#[doc = "Register `FREQ` reader"]
pub type R = crate::R<FreqSpec>;
#[doc = "Register `FREQ` writer"]
pub type W = crate::W<FreqSpec>;
#[doc = "Select CPU speed\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freq {
    #[doc = "1: 128 MHz"]
    Ck128m = 1,
    #[doc = "3: 64 MHz"]
    Ck64m = 3,
}
impl From<Freq> for u8 {
    #[inline(always)]
    fn from(variant: Freq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freq {
    type Ux = u8;
}
impl crate::IsEnum for Freq {}
#[doc = "Field `FREQ` reader - Select CPU speed"]
pub type FreqR = crate::FieldReader<Freq>;
impl FreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Freq> {
        match self.bits {
            1 => Some(Freq::Ck128m),
            3 => Some(Freq::Ck64m),
            _ => None,
        }
    }
    #[doc = "128 MHz"]
    #[inline(always)]
    pub fn is_ck128m(&self) -> bool {
        *self == Freq::Ck128m
    }
    #[doc = "64 MHz"]
    #[inline(always)]
    pub fn is_ck64m(&self) -> bool {
        *self == Freq::Ck64m
    }
}
#[doc = "Field `FREQ` writer - Select CPU speed"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Freq>;
impl<'a, REG> FreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 MHz"]
    #[inline(always)]
    pub fn ck128m(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Ck128m)
    }
    #[doc = "64 MHz"]
    #[inline(always)]
    pub fn ck64m(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Ck64m)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select CPU speed"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select CPU speed"]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<FreqSpec> {
        FreqW::new(self, 0)
    }
}
#[doc = "Set speed of MCU power domain, including CPU\n\nYou can [`read`](crate::Reg::read) this register and get [`freq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqSpec;
impl crate::RegisterSpec for FreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freq::R`](R) reader structure"]
impl crate::Readable for FreqSpec {}
#[doc = "`write(|w| ..)` method takes [`freq::W`](W) writer structure"]
impl crate::Writable for FreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQ to value 0x03"]
impl crate::Resettable for FreqSpec {
    const RESET_VALUE: u32 = 0x03;
}
