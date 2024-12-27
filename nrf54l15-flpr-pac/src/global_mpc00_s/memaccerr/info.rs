#[doc = "Register `INFO` reader"]
pub type R = crate::R<InfoSpec>;
#[doc = "Field `OWNERID` reader - Owner identifier of the erroneous access"]
pub type OwneridR = crate::FieldReader;
#[doc = "Field `MASTERPORT` reader - Master port where erroneous access is detected"]
pub type MasterportR = crate::FieldReader;
#[doc = "Read bit of bus access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "1: Read access bit was set"]
    Set = 1,
    #[doc = "0: Read access bit was not set"]
    NotSet = 0,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Read bit of bus access"]
pub type ReadR = crate::BitReader<Read>;
impl ReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Read {
        match self.bits {
            true => Read::Set,
            false => Read::NotSet,
        }
    }
    #[doc = "Read access bit was set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Read::Set
    }
    #[doc = "Read access bit was not set"]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Read::NotSet
    }
}
#[doc = "Write bit of bus access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "1: Write access bit was set"]
    Set = 1,
    #[doc = "0: Write access bit was not set"]
    NotSet = 0,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Write bit of bus access"]
pub type WriteR = crate::BitReader<Write>;
impl WriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Write {
        match self.bits {
            true => Write::Set,
            false => Write::NotSet,
        }
    }
    #[doc = "Write access bit was set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Write::Set
    }
    #[doc = "Write access bit was not set"]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Write::NotSet
    }
}
#[doc = "Execute bit of bus access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Execute {
    #[doc = "1: Execute access bit was set"]
    Set = 1,
    #[doc = "0: Execute access bit was not set"]
    NotSet = 0,
}
impl From<Execute> for bool {
    #[inline(always)]
    fn from(variant: Execute) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXECUTE` reader - Execute bit of bus access"]
pub type ExecuteR = crate::BitReader<Execute>;
impl ExecuteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Execute {
        match self.bits {
            true => Execute::Set,
            false => Execute::NotSet,
        }
    }
    #[doc = "Execute access bit was set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Execute::Set
    }
    #[doc = "Execute access bit was not set"]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Execute::NotSet
    }
}
#[doc = "Secure bit of bus access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secure {
    #[doc = "1: Secure access bit was set"]
    Set = 1,
    #[doc = "0: Secure access bit was not set"]
    NotSet = 0,
}
impl From<Secure> for bool {
    #[inline(always)]
    fn from(variant: Secure) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECURE` reader - Secure bit of bus access"]
pub type SecureR = crate::BitReader<Secure>;
impl SecureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secure {
        match self.bits {
            true => Secure::Set,
            false => Secure::NotSet,
        }
    }
    #[doc = "Secure access bit was set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Secure::Set
    }
    #[doc = "Secure access bit was not set"]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Secure::NotSet
    }
}
#[doc = "Source of memory access error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errorsource {
    #[doc = "1: Error was triggered by MPC module"]
    Mpc = 1,
    #[doc = "0: Error was triggered by an AXI slave"]
    Slave = 0,
}
impl From<Errorsource> for bool {
    #[inline(always)]
    fn from(variant: Errorsource) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORSOURCE` reader - Source of memory access error"]
pub type ErrorsourceR = crate::BitReader<Errorsource>;
impl ErrorsourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errorsource {
        match self.bits {
            true => Errorsource::Mpc,
            false => Errorsource::Slave,
        }
    }
    #[doc = "Error was triggered by MPC module"]
    #[inline(always)]
    pub fn is_mpc(&self) -> bool {
        *self == Errorsource::Mpc
    }
    #[doc = "Error was triggered by an AXI slave"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Errorsource::Slave
    }
}
impl R {
    #[doc = "Bits 0:3 - Owner identifier of the erroneous access"]
    #[inline(always)]
    pub fn ownerid(&self) -> OwneridR {
        OwneridR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - Master port where erroneous access is detected"]
    #[inline(always)]
    pub fn masterport(&self) -> MasterportR {
        MasterportR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - Read bit of bus access"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write bit of bus access"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Execute bit of bus access"]
    #[inline(always)]
    pub fn execute(&self) -> ExecuteR {
        ExecuteR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Secure bit of bus access"]
    #[inline(always)]
    pub fn secure(&self) -> SecureR {
        SecureR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Source of memory access error"]
    #[inline(always)]
    pub fn errorsource(&self) -> ErrorsourceR {
        ErrorsourceR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Access information for the transaction that triggered a memory access error. Register content won't be changed as long as MEMACCERR event is active.\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for InfoSpec {}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for InfoSpec {
    const RESET_VALUE: u32 = 0;
}
