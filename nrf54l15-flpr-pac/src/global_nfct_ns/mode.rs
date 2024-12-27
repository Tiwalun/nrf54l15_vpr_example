#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Enable low-power operation, or use low-latency\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpop {
    #[doc = "0: Low-latency operation"]
    LowLat = 0,
    #[doc = "1: Low-power operation"]
    LowPower = 1,
    #[doc = "3: Full Low-power operation"]
    FullLowPower = 3,
}
impl From<Lpop> for u8 {
    #[inline(always)]
    fn from(variant: Lpop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpop {
    type Ux = u8;
}
impl crate::IsEnum for Lpop {}
#[doc = "Field `LPOP` reader - Enable low-power operation, or use low-latency"]
pub type LpopR = crate::FieldReader<Lpop>;
impl LpopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lpop> {
        match self.bits {
            0 => Some(Lpop::LowLat),
            1 => Some(Lpop::LowPower),
            3 => Some(Lpop::FullLowPower),
            _ => None,
        }
    }
    #[doc = "Low-latency operation"]
    #[inline(always)]
    pub fn is_low_lat(&self) -> bool {
        *self == Lpop::LowLat
    }
    #[doc = "Low-power operation"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Lpop::LowPower
    }
    #[doc = "Full Low-power operation"]
    #[inline(always)]
    pub fn is_full_low_power(&self) -> bool {
        *self == Lpop::FullLowPower
    }
}
#[doc = "Field `LPOP` writer - Enable low-power operation, or use low-latency"]
pub type LpopW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpop>;
impl<'a, REG> LpopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low-latency operation"]
    #[inline(always)]
    pub fn low_lat(self) -> &'a mut crate::W<REG> {
        self.variant(Lpop::LowLat)
    }
    #[doc = "Low-power operation"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Lpop::LowPower)
    }
    #[doc = "Full Low-power operation"]
    #[inline(always)]
    pub fn full_low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Lpop::FullLowPower)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable low-power operation, or use low-latency"]
    #[inline(always)]
    pub fn lpop(&self) -> LpopR {
        LpopR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable low-power operation, or use low-latency"]
    #[inline(always)]
    #[must_use]
    pub fn lpop(&mut self) -> LpopW<ModeSpec> {
        LpopW::new(self, 0)
    }
}
#[doc = "Configure EasyDMA mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE to value 0x01"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0x01;
}
