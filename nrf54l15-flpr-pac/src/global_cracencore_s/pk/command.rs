#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<CommandSpec>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Field `OPEADDR` reader - This field defines the operation to be performed."]
pub type OpeaddrR = crate::FieldReader;
#[doc = "Field `OPEADDR` writer - This field defines the operation to be performed."]
pub type OpeaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FIELDF` reader - 0: Field is GF(p) 1: Field is GF(2**m)"]
pub type FieldfR = crate::BitReader;
#[doc = "Field `FIELDF` writer - 0: Field is GF(p) 1: Field is GF(2**m)"]
pub type FieldfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPBYTESM1` reader - This field defines the size (= number of bytes minus one) of the operands for the current operation."]
pub type Opbytesm1R = crate::FieldReader<u16>;
#[doc = "Field `OPBYTESM1` writer - This field defines the size (= number of bytes minus one) of the operands for the current operation."]
pub type Opbytesm1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RANDMOD` reader - Enable randomization of modulus (counter-measure)."]
pub type RandmodR = crate::BitReader;
#[doc = "Field `RANDMOD` writer - Enable randomization of modulus (counter-measure)."]
pub type RandmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable accelerator for specific curve modulus:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selcurve {
    #[doc = "0: Unspecified"]
    Noaccel = 0,
    #[doc = "1: Unspecified"]
    P256 = 1,
    #[doc = "2: Unspecified"]
    P384 = 2,
    #[doc = "3: Unspecified"]
    P521 = 3,
    #[doc = "4: Unspecified"]
    P192 = 4,
    #[doc = "5: Unspecified"]
    Curve25519 = 5,
    #[doc = "6: Unspecified"]
    Ed25519 = 6,
}
impl From<Selcurve> for u8 {
    #[inline(always)]
    fn from(variant: Selcurve) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selcurve {
    type Ux = u8;
}
impl crate::IsEnum for Selcurve {}
#[doc = "Field `SELCURVE` reader - Enable accelerator for specific curve modulus:"]
pub type SelcurveR = crate::FieldReader<Selcurve>;
impl SelcurveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selcurve> {
        match self.bits {
            0 => Some(Selcurve::Noaccel),
            1 => Some(Selcurve::P256),
            2 => Some(Selcurve::P384),
            3 => Some(Selcurve::P521),
            4 => Some(Selcurve::P192),
            5 => Some(Selcurve::Curve25519),
            6 => Some(Selcurve::Ed25519),
            _ => None,
        }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_noaccel(&self) -> bool {
        *self == Selcurve::Noaccel
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        *self == Selcurve::P256
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_p384(&self) -> bool {
        *self == Selcurve::P384
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_p521(&self) -> bool {
        *self == Selcurve::P521
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_p192(&self) -> bool {
        *self == Selcurve::P192
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_curve25519(&self) -> bool {
        *self == Selcurve::Curve25519
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_ed25519(&self) -> bool {
        *self == Selcurve::Ed25519
    }
}
#[doc = "Field `SELCURVE` writer - Enable accelerator for specific curve modulus:"]
pub type SelcurveW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selcurve>;
impl<'a, REG> SelcurveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn noaccel(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::Noaccel)
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn p256(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::P256)
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn p384(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::P384)
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn p521(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::P521)
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn p192(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::P192)
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn curve25519(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::Curve25519)
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn ed25519(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::Ed25519)
    }
}
#[doc = "Field `RANDKE` reader - Enable randomization of exponent/scalar (counter-measure)."]
pub type RandkeR = crate::BitReader;
#[doc = "Field `RANDKE` writer - Enable randomization of exponent/scalar (counter-measure)."]
pub type RandkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANDPROJ` reader - Enable randomization of projective coordinates (counter-measure)."]
pub type RandprojR = crate::BitReader;
#[doc = "Field `RANDPROJ` writer - Enable randomization of projective coordinates (counter-measure)."]
pub type RandprojW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDWARDS` reader - Enable Edwards curve."]
pub type EdwardsR = crate::BitReader;
#[doc = "Field `EDWARDS` writer - Enable Edwards curve."]
pub type EdwardsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Swap the bytes on AHB interface:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swapbytes {
    #[doc = "0: Native format (little endian)."]
    Native = 0,
    #[doc = "1: Byte swapped (big endian)."]
    Swapped = 1,
}
impl From<Swapbytes> for bool {
    #[inline(always)]
    fn from(variant: Swapbytes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAPBYTES` reader - Swap the bytes on AHB interface:"]
pub type SwapbytesR = crate::BitReader<Swapbytes>;
impl SwapbytesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swapbytes {
        match self.bits {
            false => Swapbytes::Native,
            true => Swapbytes::Swapped,
        }
    }
    #[doc = "Native format (little endian)."]
    #[inline(always)]
    pub fn is_native(&self) -> bool {
        *self == Swapbytes::Native
    }
    #[doc = "Byte swapped (big endian)."]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == Swapbytes::Swapped
    }
}
#[doc = "Field `SWAPBYTES` writer - Swap the bytes on AHB interface:"]
pub type SwapbytesW<'a, REG> = crate::BitWriter<'a, REG, Swapbytes>;
impl<'a, REG> SwapbytesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Native format (little endian)."]
    #[inline(always)]
    pub fn native(self) -> &'a mut crate::W<REG> {
        self.variant(Swapbytes::Native)
    }
    #[doc = "Byte swapped (big endian)."]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(Swapbytes::Swapped)
    }
}
#[doc = "Field `FLAGA` reader - Flag A."]
pub type FlagaR = crate::BitReader;
#[doc = "Field `FLAGA` writer - Flag A."]
pub type FlagaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAGB` reader - Flag B."]
pub type FlagbR = crate::BitReader;
#[doc = "Field `FLAGB` writer - Flag B."]
pub type FlagbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This bit indicates if the IP has to calculate R**2 mod N for the next operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calcr2 {
    #[doc = "0: don't recalculate R² mod N"]
    Nrecalculate = 0,
    #[doc = "1: re-calculate R² mod N"]
    Recalculate = 1,
}
impl From<Calcr2> for bool {
    #[inline(always)]
    fn from(variant: Calcr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALCR2` reader - This bit indicates if the IP has to calculate R**2 mod N for the next operation."]
pub type Calcr2R = crate::BitReader<Calcr2>;
impl Calcr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calcr2 {
        match self.bits {
            false => Calcr2::Nrecalculate,
            true => Calcr2::Recalculate,
        }
    }
    #[doc = "don't recalculate R² mod N"]
    #[inline(always)]
    pub fn is_nrecalculate(&self) -> bool {
        *self == Calcr2::Nrecalculate
    }
    #[doc = "re-calculate R² mod N"]
    #[inline(always)]
    pub fn is_recalculate(&self) -> bool {
        *self == Calcr2::Recalculate
    }
}
#[doc = "Field `CALCR2` writer - This bit indicates if the IP has to calculate R**2 mod N for the next operation."]
pub type Calcr2W<'a, REG> = crate::BitWriter<'a, REG, Calcr2>;
impl<'a, REG> Calcr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "don't recalculate R² mod N"]
    #[inline(always)]
    pub fn nrecalculate(self) -> &'a mut crate::W<REG> {
        self.variant(Calcr2::Nrecalculate)
    }
    #[doc = "re-calculate R² mod N"]
    #[inline(always)]
    pub fn recalculate(self) -> &'a mut crate::W<REG> {
        self.variant(Calcr2::Recalculate)
    }
}
impl R {
    #[doc = "Bits 0:6 - This field defines the operation to be performed."]
    #[inline(always)]
    pub fn opeaddr(&self) -> OpeaddrR {
        OpeaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 0: Field is GF(p) 1: Field is GF(2**m)"]
    #[inline(always)]
    pub fn fieldf(&self) -> FieldfR {
        FieldfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:17 - This field defines the size (= number of bytes minus one) of the operands for the current operation."]
    #[inline(always)]
    pub fn opbytesm1(&self) -> Opbytesm1R {
        Opbytesm1R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - Enable randomization of modulus (counter-measure)."]
    #[inline(always)]
    pub fn randmod(&self) -> RandmodR {
        RandmodR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Enable accelerator for specific curve modulus:"]
    #[inline(always)]
    pub fn selcurve(&self) -> SelcurveR {
        SelcurveR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Enable randomization of exponent/scalar (counter-measure)."]
    #[inline(always)]
    pub fn randke(&self) -> RandkeR {
        RandkeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable randomization of projective coordinates (counter-measure)."]
    #[inline(always)]
    pub fn randproj(&self) -> RandprojR {
        RandprojR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Edwards curve."]
    #[inline(always)]
    pub fn edwards(&self) -> EdwardsR {
        EdwardsR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Swap the bytes on AHB interface:"]
    #[inline(always)]
    pub fn swapbytes(&self) -> SwapbytesR {
        SwapbytesR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Flag A."]
    #[inline(always)]
    pub fn flaga(&self) -> FlagaR {
        FlagaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Flag B."]
    #[inline(always)]
    pub fn flagb(&self) -> FlagbR {
        FlagbR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit indicates if the IP has to calculate R**2 mod N for the next operation."]
    #[inline(always)]
    pub fn calcr2(&self) -> Calcr2R {
        Calcr2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - This field defines the operation to be performed."]
    #[inline(always)]
    #[must_use]
    pub fn opeaddr(&mut self) -> OpeaddrW<CommandSpec> {
        OpeaddrW::new(self, 0)
    }
    #[doc = "Bit 7 - 0: Field is GF(p) 1: Field is GF(2**m)"]
    #[inline(always)]
    #[must_use]
    pub fn fieldf(&mut self) -> FieldfW<CommandSpec> {
        FieldfW::new(self, 7)
    }
    #[doc = "Bits 8:17 - This field defines the size (= number of bytes minus one) of the operands for the current operation."]
    #[inline(always)]
    #[must_use]
    pub fn opbytesm1(&mut self) -> Opbytesm1W<CommandSpec> {
        Opbytesm1W::new(self, 8)
    }
    #[doc = "Bit 19 - Enable randomization of modulus (counter-measure)."]
    #[inline(always)]
    #[must_use]
    pub fn randmod(&mut self) -> RandmodW<CommandSpec> {
        RandmodW::new(self, 19)
    }
    #[doc = "Bits 20:22 - Enable accelerator for specific curve modulus:"]
    #[inline(always)]
    #[must_use]
    pub fn selcurve(&mut self) -> SelcurveW<CommandSpec> {
        SelcurveW::new(self, 20)
    }
    #[doc = "Bit 24 - Enable randomization of exponent/scalar (counter-measure)."]
    #[inline(always)]
    #[must_use]
    pub fn randke(&mut self) -> RandkeW<CommandSpec> {
        RandkeW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable randomization of projective coordinates (counter-measure)."]
    #[inline(always)]
    #[must_use]
    pub fn randproj(&mut self) -> RandprojW<CommandSpec> {
        RandprojW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Edwards curve."]
    #[inline(always)]
    #[must_use]
    pub fn edwards(&mut self) -> EdwardsW<CommandSpec> {
        EdwardsW::new(self, 26)
    }
    #[doc = "Bit 28 - Swap the bytes on AHB interface:"]
    #[inline(always)]
    #[must_use]
    pub fn swapbytes(&mut self) -> SwapbytesW<CommandSpec> {
        SwapbytesW::new(self, 28)
    }
    #[doc = "Bit 29 - Flag A."]
    #[inline(always)]
    #[must_use]
    pub fn flaga(&mut self) -> FlagaW<CommandSpec> {
        FlagaW::new(self, 29)
    }
    #[doc = "Bit 30 - Flag B."]
    #[inline(always)]
    #[must_use]
    pub fn flagb(&mut self) -> FlagbW<CommandSpec> {
        FlagbW::new(self, 30)
    }
    #[doc = "Bit 31 - This bit indicates if the IP has to calculate R**2 mod N for the next operation."]
    #[inline(always)]
    #[must_use]
    pub fn calcr2(&mut self) -> Calcr2W<CommandSpec> {
        Calcr2W::new(self, 31)
    }
}
#[doc = "Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0x0f"]
impl crate::Resettable for CommandSpec {
    const RESET_VALUE: u32 = 0x0f;
}
