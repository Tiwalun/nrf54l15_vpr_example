#[doc = "Register `CLKOUT` reader"]
pub type R = crate::R<ClkoutSpec>;
#[doc = "Register `CLKOUT` writer"]
pub type W = crate::W<ClkoutSpec>;
#[doc = "Enable 32Ki clock output on pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkout32k {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Clkout32k> for bool {
    #[inline(always)]
    fn from(variant: Clkout32k) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKOUT32K` reader - Enable 32Ki clock output on pin"]
pub type Clkout32kR = crate::BitReader<Clkout32k>;
impl Clkout32kR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkout32k {
        match self.bits {
            false => Clkout32k::Disabled,
            true => Clkout32k::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clkout32k::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Clkout32k::Enabled
    }
}
#[doc = "Field `CLKOUT32K` writer - Enable 32Ki clock output on pin"]
pub type Clkout32kW<'a, REG> = crate::BitWriter<'a, REG, Clkout32k>;
impl<'a, REG> Clkout32kW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkout32k::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkout32k::Enabled)
    }
}
#[doc = "Enable fast clock output on pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkoutfast {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Clkoutfast> for bool {
    #[inline(always)]
    fn from(variant: Clkoutfast) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKOUTFAST` reader - Enable fast clock output on pin"]
pub type ClkoutfastR = crate::BitReader<Clkoutfast>;
impl ClkoutfastR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkoutfast {
        match self.bits {
            false => Clkoutfast::Disabled,
            true => Clkoutfast::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clkoutfast::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Clkoutfast::Enabled
    }
}
#[doc = "Field `CLKOUTFAST` writer - Enable fast clock output on pin"]
pub type ClkoutfastW<'a, REG> = crate::BitWriter<'a, REG, Clkoutfast>;
impl<'a, REG> ClkoutfastW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutfast::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutfast::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable 32Ki clock output on pin"]
    #[inline(always)]
    pub fn clkout32k(&self) -> Clkout32kR {
        Clkout32kR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable fast clock output on pin"]
    #[inline(always)]
    pub fn clkoutfast(&self) -> ClkoutfastR {
        ClkoutfastR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable 32Ki clock output on pin"]
    #[inline(always)]
    pub fn clkout32k(&mut self) -> Clkout32kW<ClkoutSpec> {
        Clkout32kW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable fast clock output on pin"]
    #[inline(always)]
    pub fn clkoutfast(&mut self) -> ClkoutfastW<ClkoutSpec> {
        ClkoutfastW::new(self, 1)
    }
}
#[doc = "Configuration of clock output\n\nYou can [`read`](crate::Reg::read) this register and get [`clkout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoutSpec;
impl crate::RegisterSpec for ClkoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkout::R`](R) reader structure"]
impl crate::Readable for ClkoutSpec {}
#[doc = "`write(|w| ..)` method takes [`clkout::W`](W) writer structure"]
impl crate::Writable for ClkoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKOUT to value 0"]
impl crate::Resettable for ClkoutSpec {
    const RESET_VALUE: u32 = 0;
}
