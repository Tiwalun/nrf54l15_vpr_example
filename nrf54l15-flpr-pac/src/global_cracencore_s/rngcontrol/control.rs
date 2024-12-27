#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `ENABLE` reader - Enable the NDRNG."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable the NDRNG."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSREN` reader - Select between the NDRNG with asynchronous free running oscillators (when 0) and the Pseudo-Random generator with synchronous oscillators for simulation purpose (when 1)."]
pub type LfsrenR = crate::BitReader;
#[doc = "Field `LFSREN` writer - Select between the NDRNG with asynchronous free running oscillators (when 0) and the Pseudo-Random generator with synchronous oscillators for simulation purpose (when 1)."]
pub type LfsrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select input for conditioning function and continuous tests:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Testen {
    #[doc = "0: Noise source (normal mode)."]
    Normal = 0,
    #[doc = "1: Test data register (test mode)."]
    Test = 1,
}
impl From<Testen> for bool {
    #[inline(always)]
    fn from(variant: Testen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TESTEN` reader - Select input for conditioning function and continuous tests:"]
pub type TestenR = crate::BitReader<Testen>;
impl TestenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Testen {
        match self.bits {
            false => Testen::Normal,
            true => Testen::Test,
        }
    }
    #[doc = "Noise source (normal mode)."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Testen::Normal
    }
    #[doc = "Test data register (test mode)."]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == Testen::Test
    }
}
#[doc = "Field `TESTEN` writer - Select input for conditioning function and continuous tests:"]
pub type TestenW<'a, REG> = crate::BitWriter<'a, REG, Testen>;
impl<'a, REG> TestenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noise source (normal mode)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Testen::Normal)
    }
    #[doc = "Test data register (test mode)."]
    #[inline(always)]
    pub fn test(self) -> &'a mut crate::W<REG> {
        self.variant(Testen::Test)
    }
}
#[doc = "Conditioning function bypass.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condbypass {
    #[doc = "0: the conditioning function is used (normal mode)."]
    Normal = 0,
    #[doc = "1: the conditioning function is bypassed (to observe entropy source directly)."]
    Bypass = 1,
}
impl From<Condbypass> for bool {
    #[inline(always)]
    fn from(variant: Condbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONDBYPASS` reader - Conditioning function bypass."]
pub type CondbypassR = crate::BitReader<Condbypass>;
impl CondbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Condbypass {
        match self.bits {
            false => Condbypass::Normal,
            true => Condbypass::Bypass,
        }
    }
    #[doc = "the conditioning function is used (normal mode)."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Condbypass::Normal
    }
    #[doc = "the conditioning function is bypassed (to observe entropy source directly)."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Condbypass::Bypass
    }
}
#[doc = "Field `CONDBYPASS` writer - Conditioning function bypass."]
pub type CondbypassW<'a, REG> = crate::BitWriter<'a, REG, Condbypass>;
impl<'a, REG> CondbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the conditioning function is used (normal mode)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Condbypass::Normal)
    }
    #[doc = "the conditioning function is bypassed (to observe entropy source directly)."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Condbypass::Bypass)
    }
}
#[doc = "Field `INTENREP` reader - Interrupt enable for Repetition Count Test failure."]
pub type IntenrepR = crate::BitReader;
#[doc = "Field `INTENREP` writer - Interrupt enable for Repetition Count Test failure."]
pub type IntenrepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENPROP` reader - Interrupt enable for Adaptive Proportion Test failure (1024-sample window)."]
pub type IntenpropR = crate::BitReader;
#[doc = "Field `INTENPROP` writer - Interrupt enable for Adaptive Proportion Test failure (1024-sample window)."]
pub type IntenpropW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENFULL` reader - Interrupt enable for FIFO full."]
pub type IntenfullR = crate::BitReader;
#[doc = "Field `INTENFULL` writer - Interrupt enable for FIFO full."]
pub type IntenfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Software reset:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softrst {
    #[doc = "0: Normal mode."]
    Normal = 0,
    #[doc = "1: The continuous test, the conditioning function and the FIFO are reset."]
    Ctest = 1,
}
impl From<Softrst> for bool {
    #[inline(always)]
    fn from(variant: Softrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTRST` reader - Software reset:"]
pub type SoftrstR = crate::BitReader<Softrst>;
impl SoftrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softrst {
        match self.bits {
            false => Softrst::Normal,
            true => Softrst::Ctest,
        }
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Softrst::Normal
    }
    #[doc = "The continuous test, the conditioning function and the FIFO are reset."]
    #[inline(always)]
    pub fn is_ctest(&self) -> bool {
        *self == Softrst::Ctest
    }
}
#[doc = "Field `SOFTRST` writer - Software reset:"]
pub type SoftrstW<'a, REG> = crate::BitWriter<'a, REG, Softrst>;
impl<'a, REG> SoftrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Softrst::Normal)
    }
    #[doc = "The continuous test, the conditioning function and the FIFO are reset."]
    #[inline(always)]
    pub fn ctest(self) -> &'a mut crate::W<REG> {
        self.variant(Softrst::Ctest)
    }
}
#[doc = "Field `INTENPRE` reader - Interrupt enable for AIS31 preliminary noise alarm."]
pub type IntenpreR = crate::BitReader;
#[doc = "Field `INTENPRE` writer - Interrupt enable for AIS31 preliminary noise alarm."]
pub type IntenpreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENALM` reader - Interrupt enable for AIS31 noise alarm."]
pub type IntenalmR = crate::BitReader;
#[doc = "Field `INTENALM` writer - Interrupt enable for AIS31 noise alarm."]
pub type IntenalmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEACTIVEROS` reader - Force oscillators to run when FIFO is full."]
pub type ForceactiverosR = crate::BitReader;
#[doc = "Field `FORCEACTIVEROS` writer - Force oscillators to run when FIFO is full."]
pub type ForceactiverosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEALTHTESTBYPASS` reader - Bypass NIST tests such that the results of the start-up and online test do not affect the FSM state."]
pub type HealthtestbypassR = crate::BitReader;
#[doc = "Field `HEALTHTESTBYPASS` writer - Bypass NIST tests such that the results of the start-up and online test do not affect the FSM state."]
pub type HealthtestbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS31BYPASS` reader - Bypass AIS31 tests such that the results of the start-up and online tests do not affect the FSM state."]
pub type Ais31bypassR = crate::BitReader;
#[doc = "Field `AIS31BYPASS` writer - Bypass AIS31 tests such that the results of the start-up and online tests do not affect the FSM state."]
pub type Ais31bypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select input to health test module:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Healthtestsel {
    #[doc = "0: Before conditioning."]
    Before = 0,
    #[doc = "1: After conditioning."]
    After = 1,
}
impl From<Healthtestsel> for bool {
    #[inline(always)]
    fn from(variant: Healthtestsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEALTHTESTSEL` reader - Select input to health test module:"]
pub type HealthtestselR = crate::BitReader<Healthtestsel>;
impl HealthtestselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Healthtestsel {
        match self.bits {
            false => Healthtestsel::Before,
            true => Healthtestsel::After,
        }
    }
    #[doc = "Before conditioning."]
    #[inline(always)]
    pub fn is_before(&self) -> bool {
        *self == Healthtestsel::Before
    }
    #[doc = "After conditioning."]
    #[inline(always)]
    pub fn is_after(&self) -> bool {
        *self == Healthtestsel::After
    }
}
#[doc = "Field `HEALTHTESTSEL` writer - Select input to health test module:"]
pub type HealthtestselW<'a, REG> = crate::BitWriter<'a, REG, Healthtestsel>;
impl<'a, REG> HealthtestselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Before conditioning."]
    #[inline(always)]
    pub fn before(self) -> &'a mut crate::W<REG> {
        self.variant(Healthtestsel::Before)
    }
    #[doc = "After conditioning."]
    #[inline(always)]
    pub fn after(self) -> &'a mut crate::W<REG> {
        self.variant(Healthtestsel::After)
    }
}
#[doc = "Select input to the AIS31 test module:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ais31testsel {
    #[doc = "0: Before conditioning."]
    Before = 0,
    #[doc = "1: After conditioning."]
    After = 1,
}
impl From<Ais31testsel> for bool {
    #[inline(always)]
    fn from(variant: Ais31testsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIS31TESTSEL` reader - Select input to the AIS31 test module:"]
pub type Ais31testselR = crate::BitReader<Ais31testsel>;
impl Ais31testselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ais31testsel {
        match self.bits {
            false => Ais31testsel::Before,
            true => Ais31testsel::After,
        }
    }
    #[doc = "Before conditioning."]
    #[inline(always)]
    pub fn is_before(&self) -> bool {
        *self == Ais31testsel::Before
    }
    #[doc = "After conditioning."]
    #[inline(always)]
    pub fn is_after(&self) -> bool {
        *self == Ais31testsel::After
    }
}
#[doc = "Field `AIS31TESTSEL` writer - Select input to the AIS31 test module:"]
pub type Ais31testselW<'a, REG> = crate::BitWriter<'a, REG, Ais31testsel>;
impl<'a, REG> Ais31testselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Before conditioning."]
    #[inline(always)]
    pub fn before(self) -> &'a mut crate::W<REG> {
        self.variant(Ais31testsel::Before)
    }
    #[doc = "After conditioning."]
    #[inline(always)]
    pub fn after(self) -> &'a mut crate::W<REG> {
        self.variant(Ais31testsel::After)
    }
}
#[doc = "Field `NB128BITBLOCKS` reader - Number of 128 bit blocks used in AES-CBCMAC post-processing."]
pub type Nb128bitblocksR = crate::FieldReader;
#[doc = "Field `NB128BITBLOCKS` writer - Number of 128 bit blocks used in AES-CBCMAC post-processing."]
pub type Nb128bitblocksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIFOWRITESTARTUP` reader - Enable write of the samples in the FIFO during start-up."]
pub type FifowritestartupR = crate::BitReader;
#[doc = "Field `FIFOWRITESTARTUP` writer - Enable write of the samples in the FIFO during start-up."]
pub type FifowritestartupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the NDRNG."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select between the NDRNG with asynchronous free running oscillators (when 0) and the Pseudo-Random generator with synchronous oscillators for simulation purpose (when 1)."]
    #[inline(always)]
    pub fn lfsren(&self) -> LfsrenR {
        LfsrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select input for conditioning function and continuous tests:"]
    #[inline(always)]
    pub fn testen(&self) -> TestenR {
        TestenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Conditioning function bypass."]
    #[inline(always)]
    pub fn condbypass(&self) -> CondbypassR {
        CondbypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable for Repetition Count Test failure."]
    #[inline(always)]
    pub fn intenrep(&self) -> IntenrepR {
        IntenrepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable for Adaptive Proportion Test failure (1024-sample window)."]
    #[inline(always)]
    pub fn intenprop(&self) -> IntenpropR {
        IntenpropR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for FIFO full."]
    #[inline(always)]
    pub fn intenfull(&self) -> IntenfullR {
        IntenfullR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software reset:"]
    #[inline(always)]
    pub fn softrst(&self) -> SoftrstR {
        SoftrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm."]
    #[inline(always)]
    pub fn intenpre(&self) -> IntenpreR {
        IntenpreR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm."]
    #[inline(always)]
    pub fn intenalm(&self) -> IntenalmR {
        IntenalmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Force oscillators to run when FIFO is full."]
    #[inline(always)]
    pub fn forceactiveros(&self) -> ForceactiverosR {
        ForceactiverosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bypass NIST tests such that the results of the start-up and online test do not affect the FSM state."]
    #[inline(always)]
    pub fn healthtestbypass(&self) -> HealthtestbypassR {
        HealthtestbypassR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bypass AIS31 tests such that the results of the start-up and online tests do not affect the FSM state."]
    #[inline(always)]
    pub fn ais31bypass(&self) -> Ais31bypassR {
        Ais31bypassR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select input to health test module:"]
    #[inline(always)]
    pub fn healthtestsel(&self) -> HealthtestselR {
        HealthtestselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select input to the AIS31 test module:"]
    #[inline(always)]
    pub fn ais31testsel(&self) -> Ais31testselR {
        Ais31testselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Number of 128 bit blocks used in AES-CBCMAC post-processing."]
    #[inline(always)]
    pub fn nb128bitblocks(&self) -> Nb128bitblocksR {
        Nb128bitblocksR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Enable write of the samples in the FIFO during start-up."]
    #[inline(always)]
    pub fn fifowritestartup(&self) -> FifowritestartupR {
        FifowritestartupR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the NDRNG."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<ControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Select between the NDRNG with asynchronous free running oscillators (when 0) and the Pseudo-Random generator with synchronous oscillators for simulation purpose (when 1)."]
    #[inline(always)]
    pub fn lfsren(&mut self) -> LfsrenW<ControlSpec> {
        LfsrenW::new(self, 1)
    }
    #[doc = "Bit 2 - Select input for conditioning function and continuous tests:"]
    #[inline(always)]
    pub fn testen(&mut self) -> TestenW<ControlSpec> {
        TestenW::new(self, 2)
    }
    #[doc = "Bit 3 - Conditioning function bypass."]
    #[inline(always)]
    pub fn condbypass(&mut self) -> CondbypassW<ControlSpec> {
        CondbypassW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable for Repetition Count Test failure."]
    #[inline(always)]
    pub fn intenrep(&mut self) -> IntenrepW<ControlSpec> {
        IntenrepW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable for Adaptive Proportion Test failure (1024-sample window)."]
    #[inline(always)]
    pub fn intenprop(&mut self) -> IntenpropW<ControlSpec> {
        IntenpropW::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt enable for FIFO full."]
    #[inline(always)]
    pub fn intenfull(&mut self) -> IntenfullW<ControlSpec> {
        IntenfullW::new(self, 7)
    }
    #[doc = "Bit 8 - Software reset:"]
    #[inline(always)]
    pub fn softrst(&mut self) -> SoftrstW<ControlSpec> {
        SoftrstW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm."]
    #[inline(always)]
    pub fn intenpre(&mut self) -> IntenpreW<ControlSpec> {
        IntenpreW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm."]
    #[inline(always)]
    pub fn intenalm(&mut self) -> IntenalmW<ControlSpec> {
        IntenalmW::new(self, 10)
    }
    #[doc = "Bit 11 - Force oscillators to run when FIFO is full."]
    #[inline(always)]
    pub fn forceactiveros(&mut self) -> ForceactiverosW<ControlSpec> {
        ForceactiverosW::new(self, 11)
    }
    #[doc = "Bit 12 - Bypass NIST tests such that the results of the start-up and online test do not affect the FSM state."]
    #[inline(always)]
    pub fn healthtestbypass(&mut self) -> HealthtestbypassW<ControlSpec> {
        HealthtestbypassW::new(self, 12)
    }
    #[doc = "Bit 13 - Bypass AIS31 tests such that the results of the start-up and online tests do not affect the FSM state."]
    #[inline(always)]
    pub fn ais31bypass(&mut self) -> Ais31bypassW<ControlSpec> {
        Ais31bypassW::new(self, 13)
    }
    #[doc = "Bit 14 - Select input to health test module:"]
    #[inline(always)]
    pub fn healthtestsel(&mut self) -> HealthtestselW<ControlSpec> {
        HealthtestselW::new(self, 14)
    }
    #[doc = "Bit 15 - Select input to the AIS31 test module:"]
    #[inline(always)]
    pub fn ais31testsel(&mut self) -> Ais31testselW<ControlSpec> {
        Ais31testselW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Number of 128 bit blocks used in AES-CBCMAC post-processing."]
    #[inline(always)]
    pub fn nb128bitblocks(&mut self) -> Nb128bitblocksW<ControlSpec> {
        Nb128bitblocksW::new(self, 16)
    }
    #[doc = "Bit 20 - Enable write of the samples in the FIFO during start-up."]
    #[inline(always)]
    pub fn fifowritestartup(&mut self) -> FifowritestartupW<ControlSpec> {
        FifowritestartupW::new(self, 20)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0x0004_0000"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x0004_0000;
}
