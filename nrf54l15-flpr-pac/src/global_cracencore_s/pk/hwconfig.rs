#[doc = "Register `HWCONFIG` reader"]
pub type R = crate::R<HwconfigSpec>;
#[doc = "Register `HWCONFIG` writer"]
pub type W = crate::W<HwconfigSpec>;
#[doc = "Field `MAXOPSIZE` reader - Maximum operand size (number of bytes)."]
pub type MaxopsizeR = crate::FieldReader<u16>;
#[doc = "Number of multipliers:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nbmult {
    #[doc = "0: 1 multiplier"]
    Mult1 = 0,
    #[doc = "1: 4 multipliers"]
    Mult4 = 1,
    #[doc = "2: 16 multipliers"]
    Mult16 = 2,
    #[doc = "4: 64 multipliers"]
    Mult64 = 4,
    #[doc = "8: 256 multipliers"]
    Mult256 = 8,
}
impl From<Nbmult> for u8 {
    #[inline(always)]
    fn from(variant: Nbmult) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nbmult {
    type Ux = u8;
}
impl crate::IsEnum for Nbmult {}
#[doc = "Field `NBMULT` reader - Number of multipliers:"]
pub type NbmultR = crate::FieldReader<Nbmult>;
impl NbmultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nbmult> {
        match self.bits {
            0 => Some(Nbmult::Mult1),
            1 => Some(Nbmult::Mult4),
            2 => Some(Nbmult::Mult16),
            4 => Some(Nbmult::Mult64),
            8 => Some(Nbmult::Mult256),
            _ => None,
        }
    }
    #[doc = "1 multiplier"]
    #[inline(always)]
    pub fn is_mult1(&self) -> bool {
        *self == Nbmult::Mult1
    }
    #[doc = "4 multipliers"]
    #[inline(always)]
    pub fn is_mult4(&self) -> bool {
        *self == Nbmult::Mult4
    }
    #[doc = "16 multipliers"]
    #[inline(always)]
    pub fn is_mult16(&self) -> bool {
        *self == Nbmult::Mult16
    }
    #[doc = "64 multipliers"]
    #[inline(always)]
    pub fn is_mult64(&self) -> bool {
        *self == Nbmult::Mult64
    }
    #[doc = "256 multipliers"]
    #[inline(always)]
    pub fn is_mult256(&self) -> bool {
        *self == Nbmult::Mult256
    }
}
#[doc = "Field `PRIMEFIELD` reader - Support prime field."]
pub type PrimefieldR = crate::BitReader;
#[doc = "Field `BINARYFIELD` reader - Support binary field."]
pub type BinaryfieldR = crate::BitReader;
#[doc = "Field `ECC` reader - Support error correction."]
pub type EccR = crate::BitReader;
#[doc = "Field `P256` reader - Support ECC P256 acceleration."]
pub type P256R = crate::BitReader;
#[doc = "Field `P384` reader - Support ECC P384 acceleration."]
pub type P384R = crate::BitReader;
#[doc = "Field `P521` reader - Support ECC P521 acceleration."]
pub type P521R = crate::BitReader;
#[doc = "Field `P192` reader - Support ECC P192 acceleration."]
pub type P192R = crate::BitReader;
#[doc = "Field `X25519` reader - Support Curve25519/Ed25519 acceleration."]
pub type X25519R = crate::BitReader;
#[doc = "Memory access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahbmaster {
    #[doc = "0: Memory access through AHB Slave and internally in the PKE."]
    Slave = 0,
    #[doc = "1: Memory access through AHB Master, outside the PKE."]
    Master = 1,
}
impl From<Ahbmaster> for bool {
    #[inline(always)]
    fn from(variant: Ahbmaster) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHBMASTER` reader - Memory access"]
pub type AhbmasterR = crate::BitReader<Ahbmaster>;
impl AhbmasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahbmaster {
        match self.bits {
            false => Ahbmaster::Slave,
            true => Ahbmaster::Master,
        }
    }
    #[doc = "Memory access through AHB Slave and internally in the PKE."]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Ahbmaster::Slave
    }
    #[doc = "Memory access through AHB Master, outside the PKE."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Ahbmaster::Master
    }
}
#[doc = "Field `DISABLESMX` reader - State of DisableSMx input (high when SM2/SM9 operations are disabled)."]
pub type DisablesmxR = crate::BitReader;
#[doc = "Field `DISABLECLRMEM` reader - State of DisableClrMem input (high when automatic clear of the RAM after reset is disabled)."]
pub type DisableclrmemR = crate::BitReader;
#[doc = "Field `DISABLECM` reader - State of DisableCM input (high when counter-measures are disabled)."]
pub type DisablecmR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - Maximum operand size (number of bytes)."]
    #[inline(always)]
    pub fn maxopsize(&self) -> MaxopsizeR {
        MaxopsizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Number of multipliers:"]
    #[inline(always)]
    pub fn nbmult(&self) -> NbmultR {
        NbmultR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Support prime field."]
    #[inline(always)]
    pub fn primefield(&self) -> PrimefieldR {
        PrimefieldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Support binary field."]
    #[inline(always)]
    pub fn binaryfield(&self) -> BinaryfieldR {
        BinaryfieldR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Support error correction."]
    #[inline(always)]
    pub fn ecc(&self) -> EccR {
        EccR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Support ECC P256 acceleration."]
    #[inline(always)]
    pub fn p256(&self) -> P256R {
        P256R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Support ECC P384 acceleration."]
    #[inline(always)]
    pub fn p384(&self) -> P384R {
        P384R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Support ECC P521 acceleration."]
    #[inline(always)]
    pub fn p521(&self) -> P521R {
        P521R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Support ECC P192 acceleration."]
    #[inline(always)]
    pub fn p192(&self) -> P192R {
        P192R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Support Curve25519/Ed25519 acceleration."]
    #[inline(always)]
    pub fn x25519(&self) -> X25519R {
        X25519R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Memory access"]
    #[inline(always)]
    pub fn ahbmaster(&self) -> AhbmasterR {
        AhbmasterR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - State of DisableSMx input (high when SM2/SM9 operations are disabled)."]
    #[inline(always)]
    pub fn disablesmx(&self) -> DisablesmxR {
        DisablesmxR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - State of DisableClrMem input (high when automatic clear of the RAM after reset is disabled)."]
    #[inline(always)]
    pub fn disableclrmem(&self) -> DisableclrmemR {
        DisableclrmemR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - State of DisableCM input (high when counter-measures are disabled)."]
    #[inline(always)]
    pub fn disablecm(&self) -> DisablecmR {
        DisablecmR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Hardware configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hwconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwconfigSpec;
impl crate::RegisterSpec for HwconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwconfig::R`](R) reader structure"]
impl crate::Readable for HwconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`hwconfig::W`](W) writer structure"]
impl crate::Writable for HwconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCONFIG to value 0x01f7_2200"]
impl crate::Resettable for HwconfigSpec {
    const RESET_VALUE: u32 = 0x01f7_2200;
}
