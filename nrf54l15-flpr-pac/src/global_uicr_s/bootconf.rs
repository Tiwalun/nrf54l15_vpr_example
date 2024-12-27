#[doc = "Register `BOOTCONF` reader"]
pub type R = crate::R<BootconfSpec>;
#[doc = "Register `BOOTCONF` writer"]
pub type W = crate::W<BootconfSpec>;
#[doc = "Read access\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Reading from the region is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Reading from the region is allowed"]
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
    #[doc = "Reading from the region is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Read::NotAllowed
    }
    #[doc = "Reading from the region is allowed"]
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
    #[doc = "Reading from the region is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Read::NotAllowed)
    }
    #[doc = "Reading from the region is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Allowed)
    }
}
#[doc = "Write access\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Writing to the region is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Writing to the region is allowed"]
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
    #[doc = "Writing to the region is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Write::NotAllowed
    }
    #[doc = "Writing to the region is allowed"]
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
    #[doc = "Writing to the region is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Write::NotAllowed)
    }
    #[doc = "Writing to the region is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Allowed)
    }
}
#[doc = "Execute access\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Execute {
    #[doc = "0: Executing code from the region is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Executing code from the region is allowed"]
    Allowed = 1,
}
impl From<Execute> for bool {
    #[inline(always)]
    fn from(variant: Execute) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXECUTE` reader - Execute access"]
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
    #[doc = "Executing code from the region is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Execute::NotAllowed
    }
    #[doc = "Executing code from the region is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Execute::Allowed
    }
}
#[doc = "Field `EXECUTE` writer - Execute access"]
pub type ExecuteW<'a, REG> = crate::BitWriter<'a, REG, Execute>;
impl<'a, REG> ExecuteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Executing code from the region is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::NotAllowed)
    }
    #[doc = "Executing code from the region is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::Allowed)
    }
}
#[doc = "Secure access\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secure {
    #[doc = "0: Both secure and non-secure access to region is allowed"]
    NonSecure = 0,
    #[doc = "1: Only secure access to region is allowed"]
    Secure = 1,
}
impl From<Secure> for bool {
    #[inline(always)]
    fn from(variant: Secure) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECURE` reader - Secure access"]
pub type SecureR = crate::BitReader<Secure>;
impl SecureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secure {
        match self.bits {
            false => Secure::NonSecure,
            true => Secure::Secure,
        }
    }
    #[doc = "Both secure and non-secure access to region is allowed"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Secure::NonSecure
    }
    #[doc = "Only secure access to region is allowed"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Secure::Secure
    }
}
#[doc = "Field `SECURE` writer - Secure access"]
pub type SecureW<'a, REG> = crate::BitWriter<'a, REG, Secure>;
impl<'a, REG> SecureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Both secure and non-secure access to region is allowed"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secure::NonSecure)
    }
    #[doc = "Only secure access to region is allowed"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secure::Secure)
    }
}
#[doc = "Write-once\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Writeonce {
    #[doc = "0: Write-once disabled"]
    Disabled = 0,
    #[doc = "1: Write-once enabled"]
    Enabled = 1,
}
impl From<Writeonce> for bool {
    #[inline(always)]
    fn from(variant: Writeonce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITEONCE` reader - Write-once"]
pub type WriteonceR = crate::BitReader<Writeonce>;
impl WriteonceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Writeonce {
        match self.bits {
            false => Writeonce::Disabled,
            true => Writeonce::Enabled,
        }
    }
    #[doc = "Write-once disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Writeonce::Disabled
    }
    #[doc = "Write-once enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Writeonce::Enabled
    }
}
#[doc = "Field `WRITEONCE` writer - Write-once"]
pub type WriteonceW<'a, REG> = crate::BitWriter<'a, REG, Writeonce>;
impl<'a, REG> WriteonceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write-once disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Writeonce::Disabled)
    }
    #[doc = "Write-once enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Writeonce::Enabled)
    }
}
#[doc = "Enable lock of configuration register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Lock is disabled, and the RRAMC region configuration registers for the immutable boot region are writable."]
    Disabled = 0,
    #[doc = "1: Lock is enabled, and the RRAMC configuration registers for the immutable boot region are read-only."]
    Enabled = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Enable lock of configuration register"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Disabled,
            true => Lock::Enabled,
        }
    }
    #[doc = "Lock is disabled, and the RRAMC region configuration registers for the immutable boot region are writable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lock::Disabled
    }
    #[doc = "Lock is enabled, and the RRAMC configuration registers for the immutable boot region are read-only."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lock::Enabled
    }
}
#[doc = "Field `LOCK` writer - Enable lock of configuration register"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lock is disabled, and the RRAMC region configuration registers for the immutable boot region are writable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Disabled)
    }
    #[doc = "Lock is enabled, and the RRAMC configuration registers for the immutable boot region are read-only."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Enabled)
    }
}
#[doc = "Field `SIZE` reader - Immutable boot region size"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - Immutable boot region size"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Read access"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write access"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Execute access"]
    #[inline(always)]
    pub fn execute(&self) -> ExecuteR {
        ExecuteR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure access"]
    #[inline(always)]
    pub fn secure(&self) -> SecureR {
        SecureR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12 - Write-once"]
    #[inline(always)]
    pub fn writeonce(&self) -> WriteonceR {
        WriteonceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable lock of configuration register"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Immutable boot region size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read access"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<BootconfSpec> {
        ReadW::new(self, 0)
    }
    #[doc = "Bit 1 - Write access"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<BootconfSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Execute access"]
    #[inline(always)]
    #[must_use]
    pub fn execute(&mut self) -> ExecuteW<BootconfSpec> {
        ExecuteW::new(self, 2)
    }
    #[doc = "Bit 3 - Secure access"]
    #[inline(always)]
    #[must_use]
    pub fn secure(&mut self) -> SecureW<BootconfSpec> {
        SecureW::new(self, 3)
    }
    #[doc = "Bit 12 - Write-once"]
    #[inline(always)]
    #[must_use]
    pub fn writeonce(&mut self) -> WriteonceW<BootconfSpec> {
        WriteonceW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable lock of configuration register"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<BootconfSpec> {
        LockW::new(self, 13)
    }
    #[doc = "Bits 16:20 - Immutable boot region size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<BootconfSpec> {
        SizeW::new(self, 16)
    }
}
#[doc = "Immutable boot region configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`bootconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootconfSpec;
impl crate::RegisterSpec for BootconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootconf::R`](R) reader structure"]
impl crate::Readable for BootconfSpec {}
#[doc = "`write(|w| ..)` method takes [`bootconf::W`](W) writer structure"]
impl crate::Writable for BootconfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTCONF to value 0xffff_ffff"]
impl crate::Resettable for BootconfSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
