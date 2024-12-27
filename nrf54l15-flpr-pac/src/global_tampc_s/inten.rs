#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event TAMPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamper {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Tamper> for bool {
    #[inline(always)]
    fn from(variant: Tamper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPER` reader - Enable or disable interrupt for event TAMPER"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tamper::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tamper::Enabled
    }
}
#[doc = "Field `TAMPER` writer - Enable or disable interrupt for event TAMPER"]
pub type TamperW<'a, REG> = crate::BitWriter<'a, REG, Tamper>;
impl<'a, REG> TamperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tamper::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tamper::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event WRITEERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Writeerror {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Writeerror> for bool {
    #[inline(always)]
    fn from(variant: Writeerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITEERROR` reader - Enable or disable interrupt for event WRITEERROR"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Writeerror::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Writeerror::Enabled
    }
}
#[doc = "Field `WRITEERROR` writer - Enable or disable interrupt for event WRITEERROR"]
pub type WriteerrorW<'a, REG> = crate::BitWriter<'a, REG, Writeerror>;
impl<'a, REG> WriteerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Writeerror::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Writeerror::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event TAMPER"]
    #[inline(always)]
    pub fn tamper(&self) -> TamperR {
        TamperR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event WRITEERROR"]
    #[inline(always)]
    pub fn writeerror(&self) -> WriteerrorR {
        WriteerrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event TAMPER"]
    #[inline(always)]
    pub fn tamper(&mut self) -> TamperW<IntenSpec> {
        TamperW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event WRITEERROR"]
    #[inline(always)]
    pub fn writeerror(&mut self) -> WriteerrorW<IntenSpec> {
        WriteerrorW::new(self, 1)
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
