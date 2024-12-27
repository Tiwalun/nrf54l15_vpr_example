#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event WOKENUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wokenup {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Wokenup> for bool {
    #[inline(always)]
    fn from(variant: Wokenup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WOKENUP` reader - Write '1' to enable interrupt for event WOKENUP"]
pub type WokenupR = crate::BitReader<Wokenup>;
impl WokenupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wokenup {
        match self.bits {
            false => Wokenup::Disabled,
            true => Wokenup::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wokenup::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wokenup::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event WOKENUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WokenupWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<WokenupWO> for bool {
    #[inline(always)]
    fn from(variant: WokenupWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WOKENUP` writer - Write '1' to enable interrupt for event WOKENUP"]
pub type WokenupW<'a, REG> = crate::BitWriter<'a, REG, WokenupWO>;
impl<'a, REG> WokenupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(WokenupWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Write '1' to enable interrupt for event READY"]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::Disabled,
            true => Ready::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ready::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ready::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ReadyWO> for bool {
    #[inline(always)]
    fn from(variant: ReadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Write '1' to enable interrupt for event READY"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG, ReadyWO>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event READYNEXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readynext {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Readynext> for bool {
    #[inline(always)]
    fn from(variant: Readynext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READYNEXT` reader - Write '1' to enable interrupt for event READYNEXT"]
pub type ReadynextR = crate::BitReader<Readynext>;
impl ReadynextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readynext {
        match self.bits {
            false => Readynext::Disabled,
            true => Readynext::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Readynext::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Readynext::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event READYNEXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadynextWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ReadynextWO> for bool {
    #[inline(always)]
    fn from(variant: ReadynextWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READYNEXT` writer - Write '1' to enable interrupt for event READYNEXT"]
pub type ReadynextW<'a, REG> = crate::BitWriter<'a, REG, ReadynextWO>;
impl<'a, REG> ReadynextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ReadynextWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ACCESSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accesserror {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Accesserror> for bool {
    #[inline(always)]
    fn from(variant: Accesserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCESSERROR` reader - Write '1' to enable interrupt for event ACCESSERROR"]
pub type AccesserrorR = crate::BitReader<Accesserror>;
impl AccesserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accesserror {
        match self.bits {
            false => Accesserror::Disabled,
            true => Accesserror::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Accesserror::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Accesserror::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ACCESSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccesserrorWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<AccesserrorWO> for bool {
    #[inline(always)]
    fn from(variant: AccesserrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCESSERROR` writer - Write '1' to enable interrupt for event ACCESSERROR"]
pub type AccesserrorW<'a, REG> = crate::BitWriter<'a, REG, AccesserrorWO>;
impl<'a, REG> AccesserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(AccesserrorWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event WOKENUP"]
    #[inline(always)]
    pub fn wokenup(&self) -> WokenupR {
        WokenupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event READYNEXT"]
    #[inline(always)]
    pub fn readynext(&self) -> ReadynextR {
        ReadynextR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event ACCESSERROR"]
    #[inline(always)]
    pub fn accesserror(&self) -> AccesserrorR {
        AccesserrorR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event WOKENUP"]
    #[inline(always)]
    #[must_use]
    pub fn wokenup(&mut self) -> WokenupW<IntensetSpec> {
        WokenupW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event READY"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> ReadyW<IntensetSpec> {
        ReadyW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event READYNEXT"]
    #[inline(always)]
    #[must_use]
    pub fn readynext(&mut self) -> ReadynextW<IntensetSpec> {
        ReadynextW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event ACCESSERROR"]
    #[inline(always)]
    #[must_use]
    pub fn accesserror(&mut self) -> AccesserrorW<IntensetSpec> {
        AccesserrorW::new(self, 3)
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
