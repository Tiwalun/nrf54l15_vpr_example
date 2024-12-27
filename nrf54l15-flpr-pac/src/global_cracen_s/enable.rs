#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Enable cryptomaster\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cryptomaster {
    #[doc = "0: Cryptomaster disabled."]
    Disabled = 0,
    #[doc = "1: Cryptomaster enabled."]
    Enabled = 1,
}
impl From<Cryptomaster> for bool {
    #[inline(always)]
    fn from(variant: Cryptomaster) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRYPTOMASTER` reader - Enable cryptomaster"]
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
    #[doc = "Cryptomaster disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cryptomaster::Disabled
    }
    #[doc = "Cryptomaster enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cryptomaster::Enabled
    }
}
#[doc = "Field `CRYPTOMASTER` writer - Enable cryptomaster"]
pub type CryptomasterW<'a, REG> = crate::BitWriter<'a, REG, Cryptomaster>;
impl<'a, REG> CryptomasterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cryptomaster disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cryptomaster::Disabled)
    }
    #[doc = "Cryptomaster enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cryptomaster::Enabled)
    }
}
#[doc = "Enable RNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: RNG disabled."]
    Disabled = 0,
    #[doc = "1: RNG enabled."]
    Enabled = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` reader - Enable RNG"]
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
    #[doc = "RNG disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rng::Disabled
    }
    #[doc = "RNG enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rng::Enabled
    }
}
#[doc = "Field `RNG` writer - Enable RNG"]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG, Rng>;
impl<'a, REG> RngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RNG disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Disabled)
    }
    #[doc = "RNG enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Enabled)
    }
}
#[doc = "Enable PKE and IKG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pkeikg {
    #[doc = "0: PKE and IKG disabled."]
    Disabled = 0,
    #[doc = "1: PKE and IKG enabled."]
    Enabled = 1,
}
impl From<Pkeikg> for bool {
    #[inline(always)]
    fn from(variant: Pkeikg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKEIKG` reader - Enable PKE and IKG"]
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
    #[doc = "PKE and IKG disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pkeikg::Disabled
    }
    #[doc = "PKE and IKG enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pkeikg::Enabled
    }
}
#[doc = "Field `PKEIKG` writer - Enable PKE and IKG"]
pub type PkeikgW<'a, REG> = crate::BitWriter<'a, REG, Pkeikg>;
impl<'a, REG> PkeikgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PKE and IKG disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pkeikg::Disabled)
    }
    #[doc = "PKE and IKG enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pkeikg::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable cryptomaster"]
    #[inline(always)]
    pub fn cryptomaster(&self) -> CryptomasterR {
        CryptomasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable PKE and IKG"]
    #[inline(always)]
    pub fn pkeikg(&self) -> PkeikgR {
        PkeikgR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable cryptomaster"]
    #[inline(always)]
    #[must_use]
    pub fn cryptomaster(&mut self) -> CryptomasterW<EnableSpec> {
        CryptomasterW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RngW<EnableSpec> {
        RngW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable PKE and IKG"]
    #[inline(always)]
    #[must_use]
    pub fn pkeikg(&mut self) -> PkeikgW<EnableSpec> {
        PkeikgW::new(self, 2)
    }
}
#[doc = "Enable CRACEN peripheral modules.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0;
}
