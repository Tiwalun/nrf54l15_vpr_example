#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxready {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Rxready> for bool {
    #[inline(always)]
    fn from(variant: Rxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY` reader - Read pending status of interrupt for event RXREADY"]
pub type RxreadyR = crate::BitReader<Rxready>;
impl RxreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxready {
        match self.bits {
            false => Rxready::NotPending,
            true => Rxready::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Rxready::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Rxready::Pending
    }
}
#[doc = "Read pending status of interrupt for event TXDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdone {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Txdone> for bool {
    #[inline(always)]
    fn from(variant: Txdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDONE` reader - Read pending status of interrupt for event TXDONE"]
pub type TxdoneR = crate::BitReader<Txdone>;
impl TxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdone {
        match self.bits {
            false => Txdone::NotPending,
            true => Txdone::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Txdone::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Txdone::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&self) -> RxreadyR {
        RxreadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event TXDONE"]
    #[inline(always)]
    pub fn txdone(&self) -> TxdoneR {
        TxdoneR::new(((self.bits >> 1) & 1) != 0)
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
