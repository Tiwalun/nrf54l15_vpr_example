#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event CRYPTOMASTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cryptomaster {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Cryptomaster> for bool {
    #[inline(always)]
    fn from(variant: Cryptomaster) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRYPTOMASTER` reader - Write '1' to enable interrupt for event CRYPTOMASTER"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cryptomaster::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cryptomaster::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CRYPTOMASTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CryptomasterWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<CryptomasterWO> for bool {
    #[inline(always)]
    fn from(variant: CryptomasterWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRYPTOMASTER` writer - Write '1' to enable interrupt for event CRYPTOMASTER"]
pub type CryptomasterW<'a, REG> = crate::BitWriter<'a, REG, CryptomasterWO>;
impl<'a, REG> CryptomasterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CryptomasterWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` reader - Write '1' to enable interrupt for event RNG"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rng::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rng::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RngWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RngWO> for bool {
    #[inline(always)]
    fn from(variant: RngWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` writer - Write '1' to enable interrupt for event RNG"]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG, RngWO>;
impl<'a, REG> RngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RngWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event PKEIKG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pkeikg {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pkeikg> for bool {
    #[inline(always)]
    fn from(variant: Pkeikg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKEIKG` reader - Write '1' to enable interrupt for event PKEIKG"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pkeikg::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pkeikg::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event PKEIKG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PkeikgWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<PkeikgWO> for bool {
    #[inline(always)]
    fn from(variant: PkeikgWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKEIKG` writer - Write '1' to enable interrupt for event PKEIKG"]
pub type PkeikgW<'a, REG> = crate::BitWriter<'a, REG, PkeikgWO>;
impl<'a, REG> PkeikgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(PkeikgWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event CRYPTOMASTER"]
    #[inline(always)]
    pub fn cryptomaster(&self) -> CryptomasterR {
        CryptomasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event PKEIKG"]
    #[inline(always)]
    pub fn pkeikg(&self) -> PkeikgR {
        PkeikgR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event CRYPTOMASTER"]
    #[inline(always)]
    pub fn cryptomaster(&mut self) -> CryptomasterW<IntensetSpec> {
        CryptomasterW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RNG"]
    #[inline(always)]
    pub fn rng(&mut self) -> RngW<IntensetSpec> {
        RngW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event PKEIKG"]
    #[inline(always)]
    pub fn pkeikg(&mut self) -> PkeikgW<IntensetSpec> {
        PkeikgW::new(self, 2)
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
