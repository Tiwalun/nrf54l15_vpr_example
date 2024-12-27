#[doc = "Register `CLKCFG` reader"]
pub type R = crate::R<ClkcfgSpec>;
#[doc = "Register `CLKCFG` writer"]
pub type W = crate::W<ClkcfgSpec>;
#[doc = "Field `CLKFASTDIV` reader - Fast clock divisor value of clock output"]
pub type ClkfastdivR = crate::FieldReader;
#[doc = "Field `CLKFASTDIV` writer - Fast clock divisor value of clock output"]
pub type ClkfastdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "GRTC LFCLK clock source selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: GRTC LFCLK clock source is LFXO"]
    Lfxo = 0,
    #[doc = "1: GRTC LFCLK clock source is system LFCLK"]
    SystemLfclk = 1,
    #[doc = "2: GRTC LFCLK clock source is LFLPRC"]
    Lflprc = 2,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` writer - GRTC LFCLK clock source selection"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GRTC LFCLK clock source is LFXO"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfxo)
    }
    #[doc = "GRTC LFCLK clock source is system LFCLK"]
    #[inline(always)]
    pub fn system_lfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::SystemLfclk)
    }
    #[doc = "GRTC LFCLK clock source is LFLPRC"]
    #[inline(always)]
    pub fn lflprc(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lflprc)
    }
}
impl R {
    #[doc = "Bits 0:7 - Fast clock divisor value of clock output"]
    #[inline(always)]
    pub fn clkfastdiv(&self) -> ClkfastdivR {
        ClkfastdivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fast clock divisor value of clock output"]
    #[inline(always)]
    pub fn clkfastdiv(&mut self) -> ClkfastdivW<ClkcfgSpec> {
        ClkfastdivW::new(self, 0)
    }
    #[doc = "Bits 16:17 - GRTC LFCLK clock source selection"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<ClkcfgSpec> {
        ClkselW::new(self, 16)
    }
}
#[doc = "Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkcfgSpec;
impl crate::RegisterSpec for ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg::R`](R) reader structure"]
impl crate::Readable for ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg::W`](W) writer structure"]
impl crate::Writable for ClkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCFG to value 0x0001_0001"]
impl crate::Resettable for ClkcfgSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
