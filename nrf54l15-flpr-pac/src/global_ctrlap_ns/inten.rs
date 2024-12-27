#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxready {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Rxready> for bool {
    #[inline(always)]
    fn from(variant: Rxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY` reader - Enable or disable interrupt for event RXREADY"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxready::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxready::Enabled
    }
}
#[doc = "Field `RXREADY` writer - Enable or disable interrupt for event RXREADY"]
pub type RxreadyW<'a, REG> = crate::BitWriter<'a, REG, Rxready>;
impl<'a, REG> RxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxready::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxready::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TXDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdone {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Txdone> for bool {
    #[inline(always)]
    fn from(variant: Txdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDONE` reader - Enable or disable interrupt for event TXDONE"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdone::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdone::Enabled
    }
}
#[doc = "Field `TXDONE` writer - Enable or disable interrupt for event TXDONE"]
pub type TxdoneW<'a, REG> = crate::BitWriter<'a, REG, Txdone>;
impl<'a, REG> TxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdone::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdone::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&self) -> RxreadyR {
        RxreadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event TXDONE"]
    #[inline(always)]
    pub fn txdone(&self) -> TxdoneR {
        TxdoneR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&mut self) -> RxreadyW<IntenSpec> {
        RxreadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event TXDONE"]
    #[inline(always)]
    pub fn txdone(&mut self) -> TxdoneW<IntenSpec> {
        TxdoneW::new(self, 1)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
