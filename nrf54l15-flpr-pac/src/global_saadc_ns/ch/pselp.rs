#[doc = "Register `PSELP` reader"]
pub type R = crate::R<PselpSpec>;
#[doc = "Register `PSELP` writer"]
pub type W = crate::W<PselpSpec>;
#[doc = "Field `PIN` reader - GPIO pin selection."]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - GPIO pin selection."]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT` reader - GPIO port selection"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - GPIO port selection"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Internal input selection for analog positive input when CH\\[n\\].PSELP.CONNECT = Internal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Internal {
    #[doc = "0: Connected to the internal 0.9V analog supply rail"]
    Avdd = 0,
    #[doc = "1: Connected to the internal 0.9V digital supply rail"]
    Dvdd = 1,
    #[doc = "2: Connected to VDD"]
    Vdd = 2,
}
impl From<Internal> for u8 {
    #[inline(always)]
    fn from(variant: Internal) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Internal {
    type Ux = u8;
}
impl crate::IsEnum for Internal {}
#[doc = "Field `INTERNAL` reader - Internal input selection for analog positive input when CH\\[n\\].PSELP.CONNECT = Internal"]
pub type InternalR = crate::FieldReader<Internal>;
impl InternalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Internal> {
        match self.bits {
            0 => Some(Internal::Avdd),
            1 => Some(Internal::Dvdd),
            2 => Some(Internal::Vdd),
            _ => None,
        }
    }
    #[doc = "Connected to the internal 0.9V analog supply rail"]
    #[inline(always)]
    pub fn is_avdd(&self) -> bool {
        *self == Internal::Avdd
    }
    #[doc = "Connected to the internal 0.9V digital supply rail"]
    #[inline(always)]
    pub fn is_dvdd(&self) -> bool {
        *self == Internal::Dvdd
    }
    #[doc = "Connected to VDD"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Internal::Vdd
    }
}
#[doc = "Field `INTERNAL` writer - Internal input selection for analog positive input when CH\\[n\\].PSELP.CONNECT = Internal"]
pub type InternalW<'a, REG> = crate::FieldWriter<'a, REG, 2, Internal>;
impl<'a, REG> InternalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Connected to the internal 0.9V analog supply rail"]
    #[inline(always)]
    pub fn avdd(self) -> &'a mut crate::W<REG> {
        self.variant(Internal::Avdd)
    }
    #[doc = "Connected to the internal 0.9V digital supply rail"]
    #[inline(always)]
    pub fn dvdd(self) -> &'a mut crate::W<REG> {
        self.variant(Internal::Dvdd)
    }
    #[doc = "Connected to VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Internal::Vdd)
    }
}
#[doc = "Connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Connect {
    #[doc = "0: Not connected"]
    Nc = 0,
    #[doc = "1: Select analog input"]
    AnalogInput = 1,
    #[doc = "2: Selects internal inputs."]
    Internal = 2,
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
            2 => Some(Connect::Internal),
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
    #[doc = "Selects internal inputs."]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Connect::Internal
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
    #[doc = "Selects internal inputs."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Internal)
    }
}
impl R {
    #[doc = "Bits 0:4 - GPIO pin selection."]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - GPIO port selection"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Internal input selection for analog positive input when CH\\[n\\].PSELP.CONNECT = Internal"]
    #[inline(always)]
    pub fn internal(&self) -> InternalR {
        InternalR::new(((self.bits >> 12) & 3) as u8)
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
    pub fn pin(&mut self) -> PinW<PselpSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<PselpSpec> {
        PortW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Internal input selection for analog positive input when CH\\[n\\].PSELP.CONNECT = Internal"]
    #[inline(always)]
    #[must_use]
    pub fn internal(&mut self) -> InternalW<PselpSpec> {
        InternalW::new(self, 12)
    }
    #[doc = "Bits 30:31 - Connection"]
    #[inline(always)]
    #[must_use]
    pub fn connect(&mut self) -> ConnectW<PselpSpec> {
        ConnectW::new(self, 30)
    }
}
#[doc = "Description cluster: Input positive pin selection for CH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`pselp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselpSpec;
impl crate::RegisterSpec for PselpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselp::R`](R) reader structure"]
impl crate::Readable for PselpSpec {}
#[doc = "`write(|w| ..)` method takes [`pselp::W`](W) writer structure"]
impl crate::Writable for PselpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSELP to value 0"]
impl crate::Resettable for PselpSpec {
    const RESET_VALUE: u32 = 0;
}
