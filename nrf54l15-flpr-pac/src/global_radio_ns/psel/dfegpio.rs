#[doc = "Register `DFEGPIO[%s]` reader"]
pub type R = crate::R<DfegpioSpec>;
#[doc = "Register `DFEGPIO[%s]` writer"]
pub type W = crate::W<DfegpioSpec>;
#[doc = "Field `PIN` reader - Pin number"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - Pin number"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT` reader - Port number"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - Port number"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Connection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connect {
    #[doc = "1: Disconnect"]
    Disconnected = 1,
    #[doc = "0: Connect"]
    Connected = 0,
}
impl From<Connect> for bool {
    #[inline(always)]
    fn from(variant: Connect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECT` reader - Connection"]
pub type ConnectR = crate::BitReader<Connect>;
impl ConnectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connect {
        match self.bits {
            true => Connect::Disconnected,
            false => Connect::Connected,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Connect::Disconnected
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == Connect::Connected
    }
}
#[doc = "Field `CONNECT` writer - Connection"]
pub type ConnectW<'a, REG> = crate::BitWriter<'a, REG, Connect>;
impl<'a, REG> ConnectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Disconnected)
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Connected)
    }
}
impl R {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - Port number"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&self) -> ConnectR {
        ConnectR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<DfegpioSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bits 5:8 - Port number"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<DfegpioSpec> {
        PortW::new(self, 5)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    #[must_use]
    pub fn connect(&mut self) -> ConnectW<DfegpioSpec> {
        ConnectW::new(self, 31)
    }
}
#[doc = "Description collection: Pin select for DFE pin n\n\nYou can [`read`](crate::Reg::read) this register and get [`dfegpio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfegpio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfegpioSpec;
impl crate::RegisterSpec for DfegpioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfegpio::R`](R) reader structure"]
impl crate::Readable for DfegpioSpec {}
#[doc = "`write(|w| ..)` method takes [`dfegpio::W`](W) writer structure"]
impl crate::Writable for DfegpioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFEGPIO[%s]
to value 0xffff_ffff"]
impl crate::Resettable for DfegpioSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
