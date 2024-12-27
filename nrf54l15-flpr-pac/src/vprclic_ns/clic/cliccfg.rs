#[doc = "Register `CLICCFG` reader"]
pub type R = crate::R<CliccfgSpec>;
#[doc = "Selective interrupt hardware vectoring.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nvbits {
    #[doc = "1: Selective interrupt hardware vectoring is implemented"]
    Implemented = 1,
}
impl From<Nvbits> for bool {
    #[inline(always)]
    fn from(variant: Nvbits) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NVBITS` reader - Selective interrupt hardware vectoring."]
pub type NvbitsR = crate::BitReader<Nvbits>;
impl NvbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nvbits> {
        match self.bits {
            true => Some(Nvbits::Implemented),
            _ => None,
        }
    }
    #[doc = "Selective interrupt hardware vectoring is implemented"]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Nvbits::Implemented
    }
}
#[doc = "Interrupt level encoding.\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nlbits {
    #[doc = "8: 8 bits = interrupt levels encoded in eight bits"]
    Eight = 8,
}
impl From<Nlbits> for u8 {
    #[inline(always)]
    fn from(variant: Nlbits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nlbits {
    type Ux = u8;
}
impl crate::IsEnum for Nlbits {}
#[doc = "Field `NLBITS` reader - Interrupt level encoding."]
pub type NlbitsR = crate::FieldReader<Nlbits>;
impl NlbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nlbits> {
        match self.bits {
            8 => Some(Nlbits::Eight),
            _ => None,
        }
    }
    #[doc = "8 bits = interrupt levels encoded in eight bits"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == Nlbits::Eight
    }
}
#[doc = "Interrupt privilege mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nmbits {
    #[doc = "0: All interrupts are M-mode only"]
    ModeM = 0,
}
impl From<Nmbits> for u8 {
    #[inline(always)]
    fn from(variant: Nmbits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nmbits {
    type Ux = u8;
}
impl crate::IsEnum for Nmbits {}
#[doc = "Field `NMBITS` reader - Interrupt privilege mode."]
pub type NmbitsR = crate::FieldReader<Nmbits>;
impl NmbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nmbits> {
        match self.bits {
            0 => Some(Nmbits::ModeM),
            _ => None,
        }
    }
    #[doc = "All interrupts are M-mode only"]
    #[inline(always)]
    pub fn is_mode_m(&self) -> bool {
        *self == Nmbits::ModeM
    }
}
impl R {
    #[doc = "Bit 0 - Selective interrupt hardware vectoring."]
    #[inline(always)]
    pub fn nvbits(&self) -> NvbitsR {
        NvbitsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Interrupt level encoding."]
    #[inline(always)]
    pub fn nlbits(&self) -> NlbitsR {
        NlbitsR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Interrupt privilege mode."]
    #[inline(always)]
    pub fn nmbits(&self) -> NmbitsR {
        NmbitsR::new(((self.bits >> 5) & 3) as u8)
    }
}
#[doc = "CLIC configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`cliccfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CliccfgSpec;
impl crate::RegisterSpec for CliccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cliccfg::R`](R) reader structure"]
impl crate::Readable for CliccfgSpec {}
#[doc = "`reset()` method sets CLICCFG to value 0x11"]
impl crate::Resettable for CliccfgSpec {
    const RESET_VALUE: u32 = 0x11;
}
