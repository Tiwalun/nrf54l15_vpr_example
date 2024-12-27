#[doc = "Register `PKECOMMAND` reader"]
pub type R = crate::R<PkecommandSpec>;
#[doc = "Register `PKECOMMAND` writer"]
pub type W = crate::W<PkecommandSpec>;
#[doc = "Secure mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Securemode {
    #[doc = "0: Unspecified"]
    Deactivated = 0,
    #[doc = "1: Unspecified"]
    Activated = 1,
}
impl From<Securemode> for bool {
    #[inline(always)]
    fn from(variant: Securemode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECUREMODE` reader - Secure mode."]
pub type SecuremodeR = crate::BitReader<Securemode>;
impl SecuremodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Securemode {
        match self.bits {
            false => Securemode::Deactivated,
            true => Securemode::Activated,
        }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_deactivated(&self) -> bool {
        *self == Securemode::Deactivated
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_activated(&self) -> bool {
        *self == Securemode::Activated
    }
}
#[doc = "Field `SECUREMODE` writer - Secure mode."]
pub type SecuremodeW<'a, REG> = crate::BitWriter<'a, REG, Securemode>;
impl<'a, REG> SecuremodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn deactivated(self) -> &'a mut crate::W<REG> {
        self.variant(Securemode::Deactivated)
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn activated(self) -> &'a mut crate::W<REG> {
        self.variant(Securemode::Activated)
    }
}
#[doc = "Field `SELECTEDKEY` reader - Select Generated Private Key for PKE operation."]
pub type SelectedkeyR = crate::FieldReader;
#[doc = "Field `SELECTEDKEY` writer - Select Generated Private Key for PKE operation."]
pub type SelectedkeyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Select PKE operation with Isolated Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opsel {
    #[doc = "0: Public Key Generation"]
    Pubkey = 0,
    #[doc = "1: ECDSA Signature"]
    Ecdsa = 1,
    #[doc = "2: Point Multiplication"]
    Ptmul = 2,
}
impl From<Opsel> for u8 {
    #[inline(always)]
    fn from(variant: Opsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opsel {
    type Ux = u8;
}
impl crate::IsEnum for Opsel {}
#[doc = "Field `OPSEL` reader - Select PKE operation with Isolated Key"]
pub type OpselR = crate::FieldReader<Opsel>;
impl OpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Opsel> {
        match self.bits {
            0 => Some(Opsel::Pubkey),
            1 => Some(Opsel::Ecdsa),
            2 => Some(Opsel::Ptmul),
            _ => None,
        }
    }
    #[doc = "Public Key Generation"]
    #[inline(always)]
    pub fn is_pubkey(&self) -> bool {
        *self == Opsel::Pubkey
    }
    #[doc = "ECDSA Signature"]
    #[inline(always)]
    pub fn is_ecdsa(&self) -> bool {
        *self == Opsel::Ecdsa
    }
    #[doc = "Point Multiplication"]
    #[inline(always)]
    pub fn is_ptmul(&self) -> bool {
        *self == Opsel::Ptmul
    }
}
#[doc = "Field `OPSEL` writer - Select PKE operation with Isolated Key"]
pub type OpselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Opsel>;
impl<'a, REG> OpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Public Key Generation"]
    #[inline(always)]
    pub fn pubkey(self) -> &'a mut crate::W<REG> {
        self.variant(Opsel::Pubkey)
    }
    #[doc = "ECDSA Signature"]
    #[inline(always)]
    pub fn ecdsa(self) -> &'a mut crate::W<REG> {
        self.variant(Opsel::Ecdsa)
    }
    #[doc = "Point Multiplication"]
    #[inline(always)]
    pub fn ptmul(self) -> &'a mut crate::W<REG> {
        self.variant(Opsel::Ptmul)
    }
}
impl R {
    #[doc = "Bit 0 - Secure mode."]
    #[inline(always)]
    pub fn securemode(&self) -> SecuremodeR {
        SecuremodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Select Generated Private Key for PKE operation."]
    #[inline(always)]
    pub fn selectedkey(&self) -> SelectedkeyR {
        SelectedkeyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Select PKE operation with Isolated Key"]
    #[inline(always)]
    pub fn opsel(&self) -> OpselR {
        OpselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Secure mode."]
    #[inline(always)]
    #[must_use]
    pub fn securemode(&mut self) -> SecuremodeW<PkecommandSpec> {
        SecuremodeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select Generated Private Key for PKE operation."]
    #[inline(always)]
    #[must_use]
    pub fn selectedkey(&mut self) -> SelectedkeyW<PkecommandSpec> {
        SelectedkeyW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Select PKE operation with Isolated Key"]
    #[inline(always)]
    #[must_use]
    pub fn opsel(&mut self) -> OpselW<PkecommandSpec> {
        OpselW::new(self, 8)
    }
}
#[doc = "PKE Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkecommand::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkecommand::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkecommandSpec;
impl crate::RegisterSpec for PkecommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkecommand::R`](R) reader structure"]
impl crate::Readable for PkecommandSpec {}
#[doc = "`write(|w| ..)` method takes [`pkecommand::W`](W) writer structure"]
impl crate::Writable for PkecommandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKECOMMAND to value 0"]
impl crate::Resettable for PkecommandSpec {
    const RESET_VALUE: u32 = 0;
}
