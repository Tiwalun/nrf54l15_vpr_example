#[doc = "Register `PERMMASK` reader"]
pub type R = crate::R<PermmaskSpec>;
#[doc = "Register `PERMMASK` writer"]
pub type W = crate::W<PermmaskSpec>;
#[doc = "Read mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Permission setting READ in OVERRIDE register will not be applied"]
    Masked = 0,
    #[doc = "1: Permission setting READ in OVERRIDE register will be applied"]
    UnMasked = 1,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Read mask"]
pub type ReadR = crate::BitReader<Read>;
impl ReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Read {
        match self.bits {
            false => Read::Masked,
            true => Read::UnMasked,
        }
    }
    #[doc = "Permission setting READ in OVERRIDE register will not be applied"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Read::Masked
    }
    #[doc = "Permission setting READ in OVERRIDE register will be applied"]
    #[inline(always)]
    pub fn is_un_masked(&self) -> bool {
        *self == Read::UnMasked
    }
}
#[doc = "Field `READ` writer - Read mask"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG, Read>;
impl<'a, REG> ReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Permission setting READ in OVERRIDE register will not be applied"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Masked)
    }
    #[doc = "Permission setting READ in OVERRIDE register will be applied"]
    #[inline(always)]
    pub fn un_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Read::UnMasked)
    }
}
#[doc = "Write mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Permission setting WRITE in OVERRIDE register will not be applied"]
    Masked = 0,
    #[doc = "1: Permission setting WRITE in OVERRIDE register will be applied"]
    UnMasked = 1,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Write mask"]
pub type WriteR = crate::BitReader<Write>;
impl WriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Write {
        match self.bits {
            false => Write::Masked,
            true => Write::UnMasked,
        }
    }
    #[doc = "Permission setting WRITE in OVERRIDE register will not be applied"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Write::Masked
    }
    #[doc = "Permission setting WRITE in OVERRIDE register will be applied"]
    #[inline(always)]
    pub fn is_un_masked(&self) -> bool {
        *self == Write::UnMasked
    }
}
#[doc = "Field `WRITE` writer - Write mask"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG, Write>;
impl<'a, REG> WriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Permission setting WRITE in OVERRIDE register will not be applied"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Masked)
    }
    #[doc = "Permission setting WRITE in OVERRIDE register will be applied"]
    #[inline(always)]
    pub fn un_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Write::UnMasked)
    }
}
#[doc = "Execute mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Execute {
    #[doc = "0: Permission setting EXECUTE in OVERRIDE register will not be applied"]
    Masked = 0,
    #[doc = "1: Permission setting EXECUTE in OVERRIDE register will be applied"]
    UnMasked = 1,
}
impl From<Execute> for bool {
    #[inline(always)]
    fn from(variant: Execute) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXECUTE` reader - Execute mask"]
pub type ExecuteR = crate::BitReader<Execute>;
impl ExecuteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Execute {
        match self.bits {
            false => Execute::Masked,
            true => Execute::UnMasked,
        }
    }
    #[doc = "Permission setting EXECUTE in OVERRIDE register will not be applied"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Execute::Masked
    }
    #[doc = "Permission setting EXECUTE in OVERRIDE register will be applied"]
    #[inline(always)]
    pub fn is_un_masked(&self) -> bool {
        *self == Execute::UnMasked
    }
}
#[doc = "Field `EXECUTE` writer - Execute mask"]
pub type ExecuteW<'a, REG> = crate::BitWriter<'a, REG, Execute>;
impl<'a, REG> ExecuteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Permission setting EXECUTE in OVERRIDE register will not be applied"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::Masked)
    }
    #[doc = "Permission setting EXECUTE in OVERRIDE register will be applied"]
    #[inline(always)]
    pub fn un_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::UnMasked)
    }
}
#[doc = "Security mapping mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secattr {
    #[doc = "0: Permission setting SECATTR in OVERRIDE register will not be applied"]
    Masked = 0,
    #[doc = "1: Permission setting SECATTR in OVERRIDE register will be applied"]
    UnMasked = 1,
}
impl From<Secattr> for bool {
    #[inline(always)]
    fn from(variant: Secattr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECATTR` reader - Security mapping mask"]
pub type SecattrR = crate::BitReader<Secattr>;
impl SecattrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secattr {
        match self.bits {
            false => Secattr::Masked,
            true => Secattr::UnMasked,
        }
    }
    #[doc = "Permission setting SECATTR in OVERRIDE register will not be applied"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Secattr::Masked
    }
    #[doc = "Permission setting SECATTR in OVERRIDE register will be applied"]
    #[inline(always)]
    pub fn is_un_masked(&self) -> bool {
        *self == Secattr::UnMasked
    }
}
#[doc = "Field `SECATTR` writer - Security mapping mask"]
pub type SecattrW<'a, REG> = crate::BitWriter<'a, REG, Secattr>;
impl<'a, REG> SecattrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Permission setting SECATTR in OVERRIDE register will not be applied"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::Masked)
    }
    #[doc = "Permission setting SECATTR in OVERRIDE register will be applied"]
    #[inline(always)]
    pub fn un_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::UnMasked)
    }
}
impl R {
    #[doc = "Bit 0 - Read mask"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write mask"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Execute mask"]
    #[inline(always)]
    pub fn execute(&self) -> ExecuteR {
        ExecuteR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security mapping mask"]
    #[inline(always)]
    pub fn secattr(&self) -> SecattrR {
        SecattrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read mask"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<PermmaskSpec> {
        ReadW::new(self, 0)
    }
    #[doc = "Bit 1 - Write mask"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<PermmaskSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Execute mask"]
    #[inline(always)]
    #[must_use]
    pub fn execute(&mut self) -> ExecuteW<PermmaskSpec> {
        ExecuteW::new(self, 2)
    }
    #[doc = "Bit 3 - Security mapping mask"]
    #[inline(always)]
    #[must_use]
    pub fn secattr(&mut self) -> SecattrW<PermmaskSpec> {
        SecattrW::new(self, 3)
    }
}
#[doc = "Description cluster: Masks permission setting fields from register OVERRIDE.PERM\n\nYou can [`read`](crate::Reg::read) this register and get [`permmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`permmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PermmaskSpec;
impl crate::RegisterSpec for PermmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`permmask::R`](R) reader structure"]
impl crate::Readable for PermmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`permmask::W`](W) writer structure"]
impl crate::Writable for PermmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERMMASK to value 0"]
impl crate::Resettable for PermmaskSpec {
    const RESET_VALUE: u32 = 0;
}
