#[doc = "Register `MACSTATUS` reader"]
pub type R = crate::R<MacstatusSpec>;
#[doc = "The result of the MAC check performed during the previous decryption operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Macstatus {
    #[doc = "0: MAC check failed"]
    CheckFailed = 0,
    #[doc = "1: MAC check passed"]
    CheckPassed = 1,
}
impl From<Macstatus> for bool {
    #[inline(always)]
    fn from(variant: Macstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MACSTATUS` reader - The result of the MAC check performed during the previous decryption operation"]
pub type MacstatusR = crate::BitReader<Macstatus>;
impl MacstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Macstatus {
        match self.bits {
            false => Macstatus::CheckFailed,
            true => Macstatus::CheckPassed,
        }
    }
    #[doc = "MAC check failed"]
    #[inline(always)]
    pub fn is_check_failed(&self) -> bool {
        *self == Macstatus::CheckFailed
    }
    #[doc = "MAC check passed"]
    #[inline(always)]
    pub fn is_check_passed(&self) -> bool {
        *self == Macstatus::CheckPassed
    }
}
impl R {
    #[doc = "Bit 0 - The result of the MAC check performed during the previous decryption operation"]
    #[inline(always)]
    pub fn macstatus(&self) -> MacstatusR {
        MacstatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "MAC check result\n\nYou can [`read`](crate::Reg::read) this register and get [`macstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacstatusSpec;
impl crate::RegisterSpec for MacstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstatus::R`](R) reader structure"]
impl crate::Readable for MacstatusSpec {}
#[doc = "`reset()` method sets MACSTATUS to value 0"]
impl crate::Resettable for MacstatusSpec {
    const RESET_VALUE: u32 = 0;
}
