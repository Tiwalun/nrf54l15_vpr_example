#[doc = "Register `ISOURCE` reader"]
pub type R = crate::R<IsourceSpec>;
#[doc = "Register `ISOURCE` writer"]
pub type W = crate::W<IsourceSpec>;
#[doc = "Current source select on analog input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Isource {
    #[doc = "0: Current source disabled"]
    Off = 0,
    #[doc = "1: Current source enabled (+/- 2.5 uA)"]
    Ien2uA5 = 1,
    #[doc = "2: Current source enabled (+/- 5 uA)"]
    Ien5uA = 2,
    #[doc = "3: Current source enabled (+/- 10 uA)"]
    Ien10uA = 3,
}
impl From<Isource> for u8 {
    #[inline(always)]
    fn from(variant: Isource) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Isource {
    type Ux = u8;
}
impl crate::IsEnum for Isource {}
#[doc = "Field `ISOURCE` reader - Current source select on analog input"]
pub type IsourceR = crate::FieldReader<Isource>;
impl IsourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isource {
        match self.bits {
            0 => Isource::Off,
            1 => Isource::Ien2uA5,
            2 => Isource::Ien5uA,
            3 => Isource::Ien10uA,
            _ => unreachable!(),
        }
    }
    #[doc = "Current source disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Isource::Off
    }
    #[doc = "Current source enabled (+/- 2.5 uA)"]
    #[inline(always)]
    pub fn is_ien2u_a5(&self) -> bool {
        *self == Isource::Ien2uA5
    }
    #[doc = "Current source enabled (+/- 5 uA)"]
    #[inline(always)]
    pub fn is_ien5u_a(&self) -> bool {
        *self == Isource::Ien5uA
    }
    #[doc = "Current source enabled (+/- 10 uA)"]
    #[inline(always)]
    pub fn is_ien10u_a(&self) -> bool {
        *self == Isource::Ien10uA
    }
}
#[doc = "Field `ISOURCE` writer - Current source select on analog input"]
pub type IsourceW<'a, REG> = crate::FieldWriter<'a, REG, 2, Isource, crate::Safe>;
impl<'a, REG> IsourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Current source disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Isource::Off)
    }
    #[doc = "Current source enabled (+/- 2.5 uA)"]
    #[inline(always)]
    pub fn ien2u_a5(self) -> &'a mut crate::W<REG> {
        self.variant(Isource::Ien2uA5)
    }
    #[doc = "Current source enabled (+/- 5 uA)"]
    #[inline(always)]
    pub fn ien5u_a(self) -> &'a mut crate::W<REG> {
        self.variant(Isource::Ien5uA)
    }
    #[doc = "Current source enabled (+/- 10 uA)"]
    #[inline(always)]
    pub fn ien10u_a(self) -> &'a mut crate::W<REG> {
        self.variant(Isource::Ien10uA)
    }
}
impl R {
    #[doc = "Bits 0:1 - Current source select on analog input"]
    #[inline(always)]
    pub fn isource(&self) -> IsourceR {
        IsourceR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current source select on analog input"]
    #[inline(always)]
    pub fn isource(&mut self) -> IsourceW<IsourceSpec> {
        IsourceW::new(self, 0)
    }
}
#[doc = "Current source select on analog input\n\nYou can [`read`](crate::Reg::read) this register and get [`isource::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isource::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsourceSpec;
impl crate::RegisterSpec for IsourceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isource::R`](R) reader structure"]
impl crate::Readable for IsourceSpec {}
#[doc = "`write(|w| ..)` method takes [`isource::W`](W) writer structure"]
impl crate::Writable for IsourceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISOURCE to value 0"]
impl crate::Resettable for IsourceSpec {
    const RESET_VALUE: u32 = 0;
}
