#[doc = "Register `PSELN` reader"]
pub type R = crate::R<PselnSpec>;
#[doc = "Register `PSELN` writer"]
pub type W = crate::W<PselnSpec>;
#[doc = "Field `PIN` reader - GPIO pin selection."]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - GPIO pin selection."]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT` reader - GPIO Port selection"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - GPIO Port selection"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Connect {
    #[doc = "0: Not connected"]
    Nc = 0,
    #[doc = "1: Select analog input"]
    AnalogInput = 1,
}
impl From<Connect> for u8 {
    #[inline(always)]
    fn from(variant: Connect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Connect {
    type Ux = u8;
}
impl crate::IsEnum for Connect {}
#[doc = "Field `CONNECT` reader - Connection"]
pub type ConnectR = crate::FieldReader<Connect>;
impl ConnectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Connect> {
        match self.bits {
            0 => Some(Connect::Nc),
            1 => Some(Connect::AnalogInput),
            _ => None,
        }
    }
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == Connect::Nc
    }
    #[doc = "Select analog input"]
    #[inline(always)]
    pub fn is_analog_input(&self) -> bool {
        *self == Connect::AnalogInput
    }
}
#[doc = "Field `CONNECT` writer - Connection"]
pub type ConnectW<'a, REG> = crate::FieldWriter<'a, REG, 2, Connect>;
impl<'a, REG> ConnectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Nc)
    }
    #[doc = "Select analog input"]
    #[inline(always)]
    pub fn analog_input(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::AnalogInput)
    }
}
impl R {
    #[doc = "Bits 0:4 - GPIO pin selection."]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - GPIO Port selection"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - Connection"]
    #[inline(always)]
    pub fn connect(&self) -> ConnectR {
        ConnectR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - GPIO pin selection."]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<PselnSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - GPIO Port selection"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<PselnSpec> {
        PortW::new(self, 8)
    }
    #[doc = "Bits 30:31 - Connection"]
    #[inline(always)]
    #[must_use]
    pub fn connect(&mut self) -> ConnectW<PselnSpec> {
        ConnectW::new(self, 30)
    }
}
#[doc = "Description cluster: Input negative pin selection for CH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`pseln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pseln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselnSpec;
impl crate::RegisterSpec for PselnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pseln::R`](R) reader structure"]
impl crate::Readable for PselnSpec {}
#[doc = "`write(|w| ..)` method takes [`pseln::W`](W) writer structure"]
impl crate::Writable for PselnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSELN to value 0"]
impl crate::Resettable for PselnSpec {
    const RESET_VALUE: u32 = 0;
}
