#[doc = "Register `SRCCOPY` reader"]
pub type R = crate::R<SrccopySpec>;
#[doc = "Register `SRCCOPY` writer"]
pub type W = crate::W<SrccopySpec>;
#[doc = "Value of LFCLK.SRC register when LFCLKSTART task was triggered\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: 32.768 kHz RC oscillator"]
    Lfrc = 0,
    #[doc = "1: 32.768 kHz crystal oscillator"]
    Lfxo = 1,
    #[doc = "2: 32.768 kHz synthesized from HFCLK"]
    Lfsynt = 2,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - Value of LFCLK.SRC register when LFCLKSTART task was triggered"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            0 => Some(Src::Lfrc),
            1 => Some(Src::Lfxo),
            2 => Some(Src::Lfsynt),
            _ => None,
        }
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == Src::Lfrc
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Src::Lfxo
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn is_lfsynt(&self) -> bool {
        *self == Src::Lfsynt
    }
}
#[doc = "Field `SRC` writer - Value of LFCLK.SRC register when LFCLKSTART task was triggered"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Lfrc)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Lfxo)
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn lfsynt(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Lfsynt)
    }
}
impl R {
    #[doc = "Bits 0:1 - Value of LFCLK.SRC register when LFCLKSTART task was triggered"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Value of LFCLK.SRC register when LFCLKSTART task was triggered"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<SrccopySpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Copy of LFCLK.SRC register, set when LFCLKSTART task is triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`srccopy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srccopy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrccopySpec;
impl crate::RegisterSpec for SrccopySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srccopy::R`](R) reader structure"]
impl crate::Readable for SrccopySpec {}
#[doc = "`write(|w| ..)` method takes [`srccopy::W`](W) writer structure"]
impl crate::Writable for SrccopySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCCOPY to value 0"]
impl crate::Resettable for SrccopySpec {
    const RESET_VALUE: u32 = 0;
}
