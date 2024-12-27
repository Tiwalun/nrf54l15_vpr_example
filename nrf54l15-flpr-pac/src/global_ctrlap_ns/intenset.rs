#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxready {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxready> for bool {
    #[inline(always)]
    fn from(variant: Rxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY` reader - Write '1' to enable interrupt for event RXREADY"]
pub type RxreadyR = crate::BitReader<Rxready>;
impl RxreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxready {
        match self.bits {
            false => Rxready::Disabled,
            true => Rxready::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxready::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxready::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxreadyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RxreadyWO> for bool {
    #[inline(always)]
    fn from(variant: RxreadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY` writer - Write '1' to enable interrupt for event RXREADY"]
pub type RxreadyW<'a, REG> = crate::BitWriter<'a, REG, RxreadyWO>;
impl<'a, REG> RxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxreadyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event TXDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdone {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txdone> for bool {
    #[inline(always)]
    fn from(variant: Txdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDONE` reader - Write '1' to enable interrupt for event TXDONE"]
pub type TxdoneR = crate::BitReader<Txdone>;
impl TxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdone {
        match self.bits {
            false => Txdone::Disabled,
            true => Txdone::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdone::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdone::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event TXDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxdoneWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<TxdoneWO> for bool {
    #[inline(always)]
    fn from(variant: TxdoneWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDONE` writer - Write '1' to enable interrupt for event TXDONE"]
pub type TxdoneW<'a, REG> = crate::BitWriter<'a, REG, TxdoneWO>;
impl<'a, REG> TxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TxdoneWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&self) -> RxreadyR {
        RxreadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event TXDONE"]
    #[inline(always)]
    pub fn txdone(&self) -> TxdoneR {
        TxdoneR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&mut self) -> RxreadyW<IntensetSpec> {
        RxreadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event TXDONE"]
    #[inline(always)]
    pub fn txdone(&mut self) -> TxdoneW<IntensetSpec> {
        TxdoneW::new(self, 1)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
