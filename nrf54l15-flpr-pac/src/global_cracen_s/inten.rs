#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event CRYPTOMASTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cryptomaster {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Cryptomaster> for bool {
    #[inline(always)]
    fn from(variant: Cryptomaster) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRYPTOMASTER` reader - Enable or disable interrupt for event CRYPTOMASTER"]
pub type CryptomasterR = crate::BitReader<Cryptomaster>;
impl CryptomasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cryptomaster {
        match self.bits {
            false => Cryptomaster::Disabled,
            true => Cryptomaster::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cryptomaster::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cryptomaster::Enabled
    }
}
#[doc = "Field `CRYPTOMASTER` writer - Enable or disable interrupt for event CRYPTOMASTER"]
pub type CryptomasterW<'a, REG> = crate::BitWriter<'a, REG, Cryptomaster>;
impl<'a, REG> CryptomasterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cryptomaster::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cryptomaster::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` reader - Enable or disable interrupt for event RNG"]
pub type RngR = crate::BitReader<Rng>;
impl RngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rng {
        match self.bits {
            false => Rng::Disabled,
            true => Rng::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rng::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rng::Enabled
    }
}
#[doc = "Field `RNG` writer - Enable or disable interrupt for event RNG"]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG, Rng>;
impl<'a, REG> RngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event PKEIKG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pkeikg {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pkeikg> for bool {
    #[inline(always)]
    fn from(variant: Pkeikg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKEIKG` reader - Enable or disable interrupt for event PKEIKG"]
pub type PkeikgR = crate::BitReader<Pkeikg>;
impl PkeikgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pkeikg {
        match self.bits {
            false => Pkeikg::Disabled,
            true => Pkeikg::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pkeikg::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pkeikg::Enabled
    }
}
#[doc = "Field `PKEIKG` writer - Enable or disable interrupt for event PKEIKG"]
pub type PkeikgW<'a, REG> = crate::BitWriter<'a, REG, Pkeikg>;
impl<'a, REG> PkeikgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pkeikg::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pkeikg::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event CRYPTOMASTER"]
    #[inline(always)]
    pub fn cryptomaster(&self) -> CryptomasterR {
        CryptomasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event PKEIKG"]
    #[inline(always)]
    pub fn pkeikg(&self) -> PkeikgR {
        PkeikgR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event CRYPTOMASTER"]
    #[inline(always)]
    #[must_use]
    pub fn cryptomaster(&mut self) -> CryptomasterW<IntenSpec> {
        CryptomasterW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RngW<IntenSpec> {
        RngW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event PKEIKG"]
    #[inline(always)]
    #[must_use]
    pub fn pkeikg(&mut self) -> PkeikgW<IntenSpec> {
        PkeikgW::new(self, 2)
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
