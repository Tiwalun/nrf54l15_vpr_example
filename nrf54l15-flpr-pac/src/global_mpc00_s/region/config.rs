#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `SLAVENUMBER` reader - Target slave number for region n accesses. Slave number 0 is reserved for default slave"]
pub type SlavenumberR = crate::FieldReader;
#[doc = "Field `SLAVENUMBER` writer - Target slave number for region n accesses. Slave number 0 is reserved for default slave"]
pub type SlavenumberW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Locks the region n setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Region n settings can be updated"]
    Unlocked = 0,
    #[doc = "1: Region n settings can't be updated until next reset"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Locks the region n setting"]
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
    #[doc = "Region n settings can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Region n settings can't be updated until next reset"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
#[doc = "Field `LOCK` writer - Locks the region n setting"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Region n settings can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlocked)
    }
    #[doc = "Region n settings can't be updated until next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Locked)
    }
}
#[doc = "Region n enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Region n is not used"]
    Disabled = 0,
    #[doc = "1: Region n is used"]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Region n enable"]
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
    #[doc = "Region n is not used"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Region n is used"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - Region n enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Region n is not used"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Region n is used"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
#[doc = "Read access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Read access to region n is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Read access to region n is allowed"]
    Allowed = 1,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Read access"]
pub type ReadR = crate::BitReader<Read>;
impl ReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Read {
        match self.bits {
            false => Read::NotAllowed,
            true => Read::Allowed,
        }
    }
    #[doc = "Read access to region n is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Read::NotAllowed
    }
    #[doc = "Read access to region n is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Read::Allowed
    }
}
#[doc = "Field `READ` writer - Read access"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG, Read>;
impl<'a, REG> ReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access to region n is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Read::NotAllowed)
    }
    #[doc = "Read access to region n is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Allowed)
    }
}
#[doc = "Write access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Write access to region n is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Write access to region n is allowed"]
    Allowed = 1,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Write access"]
pub type WriteR = crate::BitReader<Write>;
impl WriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Write {
        match self.bits {
            false => Write::NotAllowed,
            true => Write::Allowed,
        }
    }
    #[doc = "Write access to region n is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Write::NotAllowed
    }
    #[doc = "Write access to region n is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Write::Allowed
    }
}
#[doc = "Field `WRITE` writer - Write access"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG, Write>;
impl<'a, REG> WriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write access to region n is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Write::NotAllowed)
    }
    #[doc = "Write access to region n is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Allowed)
    }
}
#[doc = "Software execute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Execute {
    #[doc = "0: Software execution from region n is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Software execution from region n is allowed"]
    Allowed = 1,
}
impl From<Execute> for bool {
    #[inline(always)]
    fn from(variant: Execute) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXECUTE` reader - Software execute"]
pub type ExecuteR = crate::BitReader<Execute>;
impl ExecuteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Execute {
        match self.bits {
            false => Execute::NotAllowed,
            true => Execute::Allowed,
        }
    }
    #[doc = "Software execution from region n is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Execute::NotAllowed
    }
    #[doc = "Software execution from region n is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Execute::Allowed
    }
}
#[doc = "Field `EXECUTE` writer - Software execute"]
pub type ExecuteW<'a, REG> = crate::BitWriter<'a, REG, Execute>;
impl<'a, REG> ExecuteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software execution from region n is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::NotAllowed)
    }
    #[doc = "Software execution from region n is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::Allowed)
    }
}
#[doc = "Memory security mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secattr {
    #[doc = "1: Memory is mapped in secure memory address space"]
    Secure = 1,
    #[doc = "0: Memory is mapped in non-secure memory address space"]
    NonSecure = 0,
}
impl From<Secattr> for bool {
    #[inline(always)]
    fn from(variant: Secattr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECATTR` reader - Memory security mapping"]
pub type SecattrR = crate::BitReader<Secattr>;
impl SecattrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secattr {
        match self.bits {
            true => Secattr::Secure,
            false => Secattr::NonSecure,
        }
    }
    #[doc = "Memory is mapped in secure memory address space"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Secattr::Secure
    }
    #[doc = "Memory is mapped in non-secure memory address space"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Secattr::NonSecure
    }
}
#[doc = "Field `SECATTR` writer - Memory security mapping"]
pub type SecattrW<'a, REG> = crate::BitWriter<'a, REG, Secattr>;
impl<'a, REG> SecattrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory is mapped in secure memory address space"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::Secure)
    }
    #[doc = "Memory is mapped in non-secure memory address space"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::NonSecure)
    }
}
#[doc = "Field `OWNERID` reader - Region owner identifier."]
pub type OwneridR = crate::FieldReader;
#[doc = "Field `OWNERID` writer - Region owner identifier."]
pub type OwneridW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Target slave number for region n accesses. Slave number 0 is reserved for default slave"]
    #[inline(always)]
    pub fn slavenumber(&self) -> SlavenumberR {
        SlavenumberR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Locks the region n setting"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Region n enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Read access"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write access"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software execute"]
    #[inline(always)]
    pub fn execute(&self) -> ExecuteR {
        ExecuteR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Memory security mapping"]
    #[inline(always)]
    pub fn secattr(&self) -> SecattrR {
        SecattrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Region owner identifier."]
    #[inline(always)]
    pub fn ownerid(&self) -> OwneridR {
        OwneridR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Target slave number for region n accesses. Slave number 0 is reserved for default slave"]
    #[inline(always)]
    pub fn slavenumber(&mut self) -> SlavenumberW<ConfigSpec> {
        SlavenumberW::new(self, 0)
    }
    #[doc = "Bit 8 - Locks the region n setting"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<ConfigSpec> {
        LockW::new(self, 8)
    }
    #[doc = "Bit 9 - Region n enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<ConfigSpec> {
        EnableW::new(self, 9)
    }
    #[doc = "Bit 12 - Read access"]
    #[inline(always)]
    pub fn read(&mut self) -> ReadW<ConfigSpec> {
        ReadW::new(self, 12)
    }
    #[doc = "Bit 13 - Write access"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<ConfigSpec> {
        WriteW::new(self, 13)
    }
    #[doc = "Bit 14 - Software execute"]
    #[inline(always)]
    pub fn execute(&mut self) -> ExecuteW<ConfigSpec> {
        ExecuteW::new(self, 14)
    }
    #[doc = "Bit 15 - Memory security mapping"]
    #[inline(always)]
    pub fn secattr(&mut self) -> SecattrW<ConfigSpec> {
        SecattrW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Region owner identifier."]
    #[inline(always)]
    pub fn ownerid(&mut self) -> OwneridW<ConfigSpec> {
        OwneridW::new(self, 16)
    }
}
#[doc = "Description cluster: Slave region n Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
