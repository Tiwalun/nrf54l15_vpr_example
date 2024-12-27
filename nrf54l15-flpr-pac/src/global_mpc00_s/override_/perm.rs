#[doc = "Register `PERM` reader"]
pub type R = crate::R<PermSpec>;
#[doc = "Register `PERM` writer"]
pub type W = crate::W<PermSpec>;
#[doc = "Read access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Read access to override region n is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Read access to override region n is allowed"]
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
    #[doc = "Read access to override region n is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Read::NotAllowed
    }
    #[doc = "Read access to override region n is allowed"]
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
    #[doc = "Read access to override region n is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Read::NotAllowed)
    }
    #[doc = "Read access to override region n is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Allowed)
    }
}
#[doc = "Write access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Write access to override region n is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Write access to override region n is allowed"]
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
    #[doc = "Write access to override region n is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Write::NotAllowed
    }
    #[doc = "Write access to override region n is allowed"]
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
    #[doc = "Write access to override region n is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Write::NotAllowed)
    }
    #[doc = "Write access to override region n is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Allowed)
    }
}
#[doc = "Software execute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Execute {
    #[doc = "0: Software execution from override region n is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Software execution from override region n is allowed"]
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
    #[doc = "Software execution from override region n is not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Execute::NotAllowed
    }
    #[doc = "Software execution from override region n is allowed"]
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
    #[doc = "Software execution from override region n is not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::NotAllowed)
    }
    #[doc = "Software execution from override region n is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::Allowed)
    }
}
#[doc = "Security mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secattr {
    #[doc = "1: Override region n is mapped in secure memory address space"]
    Secure = 1,
    #[doc = "0: Override region n is mapped in non-secure memory address space"]
    NonSecure = 0,
}
impl From<Secattr> for bool {
    #[inline(always)]
    fn from(variant: Secattr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECATTR` reader - Security mapping"]
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
    #[doc = "Override region n is mapped in secure memory address space"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Secattr::Secure
    }
    #[doc = "Override region n is mapped in non-secure memory address space"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Secattr::NonSecure
    }
}
#[doc = "Field `SECATTR` writer - Security mapping"]
pub type SecattrW<'a, REG> = crate::BitWriter<'a, REG, Secattr>;
impl<'a, REG> SecattrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is mapped in secure memory address space"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::Secure)
    }
    #[doc = "Override region n is mapped in non-secure memory address space"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::NonSecure)
    }
}
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
    #[doc = "Bit 2 - Software execute"]
    #[inline(always)]
    pub fn execute(&self) -> ExecuteR {
        ExecuteR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security mapping"]
    #[inline(always)]
    pub fn secattr(&self) -> SecattrR {
        SecattrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read access"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<PermSpec> {
        ReadW::new(self, 0)
    }
    #[doc = "Bit 1 - Write access"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<PermSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Software execute"]
    #[inline(always)]
    #[must_use]
    pub fn execute(&mut self) -> ExecuteW<PermSpec> {
        ExecuteW::new(self, 2)
    }
    #[doc = "Bit 3 - Security mapping"]
    #[inline(always)]
    #[must_use]
    pub fn secattr(&mut self) -> SecattrW<PermSpec> {
        SecattrW::new(self, 3)
    }
}
#[doc = "Description cluster: Permission settings for override region n\n\nYou can [`read`](crate::Reg::read) this register and get [`perm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PermSpec;
impl crate::RegisterSpec for PermSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perm::R`](R) reader structure"]
impl crate::Readable for PermSpec {}
#[doc = "`write(|w| ..)` method takes [`perm::W`](W) writer structure"]
impl crate::Writable for PermSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERM to value 0"]
impl crate::Resettable for PermSpec {
    const RESET_VALUE: u32 = 0;
}
