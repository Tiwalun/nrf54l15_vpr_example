#[doc = "Register `OPSIZE` reader"]
pub type R = crate::R<OpsizeSpec>;
#[doc = "Register `OPSIZE` writer"]
pub type W = crate::W<OpsizeSpec>;
#[doc = "Operand size (number of bytes): This register is used when the memory is accessed via AHB Master\n\nValue on reset: 4096"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Opsize {
    #[doc = "256: 256 bytes."]
    Opsize256 = 256,
    #[doc = "521: 521 bytes."]
    Opsize521 = 521,
    #[doc = "2048: 2048 bytes."]
    Opsize2048 = 2048,
    #[doc = "3072: 3072 bytes."]
    Opsize3072 = 3072,
    #[doc = "4096: 4096 bytes."]
    Opsize4096 = 4096,
}
impl From<Opsize> for u16 {
    #[inline(always)]
    fn from(variant: Opsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opsize {
    type Ux = u16;
}
impl crate::IsEnum for Opsize {}
#[doc = "Field `OPSIZE` reader - Operand size (number of bytes): This register is used when the memory is accessed via AHB Master"]
pub type OpsizeR = crate::FieldReader<Opsize>;
impl OpsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Opsize> {
        match self.bits {
            256 => Some(Opsize::Opsize256),
            521 => Some(Opsize::Opsize521),
            2048 => Some(Opsize::Opsize2048),
            3072 => Some(Opsize::Opsize3072),
            4096 => Some(Opsize::Opsize4096),
            _ => None,
        }
    }
    #[doc = "256 bytes."]
    #[inline(always)]
    pub fn is_opsize256(&self) -> bool {
        *self == Opsize::Opsize256
    }
    #[doc = "521 bytes."]
    #[inline(always)]
    pub fn is_opsize521(&self) -> bool {
        *self == Opsize::Opsize521
    }
    #[doc = "2048 bytes."]
    #[inline(always)]
    pub fn is_opsize2048(&self) -> bool {
        *self == Opsize::Opsize2048
    }
    #[doc = "3072 bytes."]
    #[inline(always)]
    pub fn is_opsize3072(&self) -> bool {
        *self == Opsize::Opsize3072
    }
    #[doc = "4096 bytes."]
    #[inline(always)]
    pub fn is_opsize4096(&self) -> bool {
        *self == Opsize::Opsize4096
    }
}
#[doc = "Field `OPSIZE` writer - Operand size (number of bytes): This register is used when the memory is accessed via AHB Master"]
pub type OpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 13, Opsize>;
impl<'a, REG> OpsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "256 bytes."]
    #[inline(always)]
    pub fn opsize256(self) -> &'a mut crate::W<REG> {
        self.variant(Opsize::Opsize256)
    }
    #[doc = "521 bytes."]
    #[inline(always)]
    pub fn opsize521(self) -> &'a mut crate::W<REG> {
        self.variant(Opsize::Opsize521)
    }
    #[doc = "2048 bytes."]
    #[inline(always)]
    pub fn opsize2048(self) -> &'a mut crate::W<REG> {
        self.variant(Opsize::Opsize2048)
    }
    #[doc = "3072 bytes."]
    #[inline(always)]
    pub fn opsize3072(self) -> &'a mut crate::W<REG> {
        self.variant(Opsize::Opsize3072)
    }
    #[doc = "4096 bytes."]
    #[inline(always)]
    pub fn opsize4096(self) -> &'a mut crate::W<REG> {
        self.variant(Opsize::Opsize4096)
    }
}
impl R {
    #[doc = "Bits 0:12 - Operand size (number of bytes): This register is used when the memory is accessed via AHB Master"]
    #[inline(always)]
    pub fn opsize(&self) -> OpsizeR {
        OpsizeR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Operand size (number of bytes): This register is used when the memory is accessed via AHB Master"]
    #[inline(always)]
    pub fn opsize(&mut self) -> OpsizeW<OpsizeSpec> {
        OpsizeW::new(self, 0)
    }
}
#[doc = "Operand size register.\n\nYou can [`read`](crate::Reg::read) this register and get [`opsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpsizeSpec;
impl crate::RegisterSpec for OpsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opsize::R`](R) reader structure"]
impl crate::Readable for OpsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`opsize::W`](W) writer structure"]
impl crate::Writable for OpsizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPSIZE to value 0x1000"]
impl crate::Resettable for OpsizeSpec {
    const RESET_VALUE: u32 = 0x1000;
}
