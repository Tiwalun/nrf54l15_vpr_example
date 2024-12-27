#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event TAMPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamper {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Tamper> for bool {
    #[inline(always)]
    fn from(variant: Tamper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPER` reader - Write '1' to disable interrupt for event TAMPER"]
pub type TamperR = crate::BitReader<Tamper>;
impl TamperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamper {
        match self.bits {
            false => Tamper::Disabled,
            true => Tamper::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tamper::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tamper::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TAMPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TamperWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<TamperWO> for bool {
    #[inline(always)]
    fn from(variant: TamperWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPER` writer - Write '1' to disable interrupt for event TAMPER"]
pub type TamperW<'a, REG> = crate::BitWriter<'a, REG, TamperWO>;
impl<'a, REG> TamperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TamperWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event WRITEERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Writeerror {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Writeerror> for bool {
    #[inline(always)]
    fn from(variant: Writeerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITEERROR` reader - Write '1' to disable interrupt for event WRITEERROR"]
pub type WriteerrorR = crate::BitReader<Writeerror>;
impl WriteerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Writeerror {
        match self.bits {
            false => Writeerror::Disabled,
            true => Writeerror::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Writeerror::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Writeerror::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event WRITEERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteerrorWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<WriteerrorWO> for bool {
    #[inline(always)]
    fn from(variant: WriteerrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITEERROR` writer - Write '1' to disable interrupt for event WRITEERROR"]
pub type WriteerrorW<'a, REG> = crate::BitWriter<'a, REG, WriteerrorWO>;
impl<'a, REG> WriteerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WriteerrorWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event TAMPER"]
    #[inline(always)]
    pub fn tamper(&self) -> TamperR {
        TamperR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event WRITEERROR"]
    #[inline(always)]
    pub fn writeerror(&self) -> WriteerrorR {
        WriteerrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event TAMPER"]
    #[inline(always)]
    #[must_use]
    pub fn tamper(&mut self) -> TamperW<IntenclrSpec> {
        TamperW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event WRITEERROR"]
    #[inline(always)]
    #[must_use]
    pub fn writeerror(&mut self) -> WriteerrorW<IntenclrSpec> {
        WriteerrorW::new(self, 1)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
