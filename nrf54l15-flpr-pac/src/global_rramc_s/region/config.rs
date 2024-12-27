#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Read access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Read access to override region \\[n\\]
is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Read access to override region \\[n\\]
is allowed"]
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
    #[doc = "Read access to override region \\[n\\]
is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Read::NotAllowed
    }
    #[doc = "Read access to override region \\[n\\]
is allowed"]
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
    #[doc = "Read access to override region \\[n\\]
is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Read::NotAllowed)
    }
    #[doc = "Read access to override region \\[n\\]
is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Allowed)
    }
}
#[doc = "Write access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Write access to override region \\[n\\]
is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Write access to override region \\[n\\]
is allowed"]
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
    #[doc = "Write access to override region \\[n\\]
is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Write::NotAllowed
    }
    #[doc = "Write access to override region \\[n\\]
is allowed"]
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
    #[doc = "Write access to override region \\[n\\]
is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Write::NotAllowed)
    }
    #[doc = "Write access to override region \\[n\\]
is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Allowed)
    }
}
#[doc = "Execute access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Execute {
    #[doc = "0: Execute access to override region \\[n\\]
is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Execute access to override region \\[n\\]
is allowed"]
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
    #[doc = "Execute access to override region \\[n\\]
is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Execute::NotAllowed
    }
    #[doc = "Execute access to override region \\[n\\]
is allowed"]
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
    #[doc = "Execute access to override region \\[n\\]
is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::NotAllowed)
    }
    #[doc = "Execute access to override region \\[n\\]
is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::Allowed)
    }
}
#[doc = "Secure access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secure {
    #[doc = "0: Both Secure and non-Secure access to override region \\[n\\]
is allowed"]
    NonSecure = 0,
    #[doc = "1: Only secure access to override region \\[n\\]
is allowed"]
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
    #[doc = "Both Secure and non-Secure access to override region \\[n\\]
is allowed"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Secure::NonSecure
    }
    #[doc = "Only secure access to override region \\[n\\]
is allowed"]
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
    #[doc = "Both Secure and non-Secure access to override region \\[n\\]
is allowed"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secure::NonSecure)
    }
    #[doc = "Only secure access to override region \\[n\\]
is allowed"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secure::Secure)
    }
}
#[doc = "Owner ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Owner {
    #[doc = "0: Owner ID protection is not enforced"]
    NotEnforced = 0,
}
impl From<Owner> for u8 {
    #[inline(always)]
    fn from(variant: Owner) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Owner {
    type Ux = u8;
}
impl crate::IsEnum for Owner {}
#[doc = "Field `OWNER` reader - Owner ID"]
pub type OwnerR = crate::FieldReader<Owner>;
impl OwnerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Owner> {
        match self.bits {
            0 => Some(Owner::NotEnforced),
            _ => None,
        }
    }
    #[doc = "Owner ID protection is not enforced"]
    #[inline(always)]
    pub fn is_not_enforced(&self) -> bool {
        *self == Owner::NotEnforced
    }
}
#[doc = "Field `OWNER` writer - Owner ID"]
pub type OwnerW<'a, REG> = crate::FieldWriter<'a, REG, 4, Owner>;
impl<'a, REG> OwnerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Owner ID protection is not enforced"]
    #[inline(always)]
    pub fn not_enforced(self) -> &'a mut crate::W<REG> {
        self.variant(Owner::NotEnforced)
    }
}
#[doc = "Write-once\n\nValue on reset: 0"]
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
#[doc = "Enable lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Lock disabled for region \\[n\\]"]
    Disabled = 0,
    #[doc = "1: Lock enabled for region \\[n\\]"]
    Enabled = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Enable lock"]
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
    #[doc = "Lock disabled for region \\[n\\]"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lock::Disabled
    }
    #[doc = "Lock enabled for region \\[n\\]"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lock::Enabled
    }
}
#[doc = "Field `LOCK` writer - Enable lock"]
pub type LockW<'a, REG> = crate::BitWriter1S<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lock disabled for region \\[n\\]"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Disabled)
    }
    #[doc = "Lock enabled for region \\[n\\]"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Enabled)
    }
}
#[doc = "Field `SIZE` reader - Size in KBytes of region \\[n\\]"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - Size in KBytes of region \\[n\\]"]
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
    #[doc = "Bits 4:7 - Owner ID"]
    #[inline(always)]
    pub fn owner(&self) -> OwnerR {
        OwnerR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Write-once"]
    #[inline(always)]
    pub fn writeonce(&self) -> WriteonceR {
        WriteonceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Size in KBytes of region \\[n\\]"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read access"]
    #[inline(always)]
    pub fn read(&mut self) -> ReadW<ConfigSpec> {
        ReadW::new(self, 0)
    }
    #[doc = "Bit 1 - Write access"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<ConfigSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Execute access"]
    #[inline(always)]
    pub fn execute(&mut self) -> ExecuteW<ConfigSpec> {
        ExecuteW::new(self, 2)
    }
    #[doc = "Bit 3 - Secure access"]
    #[inline(always)]
    pub fn secure(&mut self) -> SecureW<ConfigSpec> {
        SecureW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Owner ID"]
    #[inline(always)]
    pub fn owner(&mut self) -> OwnerW<ConfigSpec> {
        OwnerW::new(self, 4)
    }
    #[doc = "Bit 12 - Write-once"]
    #[inline(always)]
    pub fn writeonce(&mut self) -> WriteonceW<ConfigSpec> {
        WriteonceW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<ConfigSpec> {
        LockW::new(self, 13)
    }
    #[doc = "Bits 16:20 - Size in KBytes of region \\[n\\]"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<ConfigSpec> {
        SizeW::new(self, 16)
    }
}
#[doc = "Description cluster: Region configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2000;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
