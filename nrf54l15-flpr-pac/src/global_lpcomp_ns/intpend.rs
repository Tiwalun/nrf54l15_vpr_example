#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Read pending status of interrupt for event READY"]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::NotPending,
            true => Ready::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Ready::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Ready::Pending
    }
}
#[doc = "Read pending status of interrupt for event DOWN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Down {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Down> for bool {
    #[inline(always)]
    fn from(variant: Down) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN` reader - Read pending status of interrupt for event DOWN"]
pub type DownR = crate::BitReader<Down>;
impl DownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Down {
        match self.bits {
            false => Down::NotPending,
            true => Down::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Down::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Down::Pending
    }
}
#[doc = "Read pending status of interrupt for event UP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Up {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Up> for bool {
    #[inline(always)]
    fn from(variant: Up) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP` reader - Read pending status of interrupt for event UP"]
pub type UpR = crate::BitReader<Up>;
impl UpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Up {
        match self.bits {
            false => Up::NotPending,
            true => Up::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Up::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Up::Pending
    }
}
#[doc = "Read pending status of interrupt for event CROSS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cross {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Cross> for bool {
    #[inline(always)]
    fn from(variant: Cross) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROSS` reader - Read pending status of interrupt for event CROSS"]
pub type CrossR = crate::BitReader<Cross>;
impl CrossR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cross {
        match self.bits {
            false => Cross::NotPending,
            true => Cross::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Cross::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Cross::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event DOWN"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event UP"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event CROSS"]
    #[inline(always)]
    pub fn cross(&self) -> CrossR {
        CrossR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpendSpec;
impl crate::RegisterSpec for IntpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpend::R`](R) reader structure"]
impl crate::Readable for IntpendSpec {}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for IntpendSpec {
    const RESET_VALUE: u32 = 0;
}
