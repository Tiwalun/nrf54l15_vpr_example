#[doc = "Register `RATEOVERRIDE` reader"]
pub type R = crate::R<RateoverrideSpec>;
#[doc = "Register `RATEOVERRIDE` writer"]
pub type W = crate::W<RateoverrideSpec>;
#[doc = "Data rate override setting.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rateoverride {
    #[doc = "0: 125 Kbps"]
    _125kbit = 0,
    #[doc = "2: 500 Kbps"]
    _500kbit = 2,
    #[doc = "3: 1 Mbps"]
    _1mbit = 3,
    #[doc = "4: 2 Mbps"]
    _2mbit = 4,
    #[doc = "5: 4 Mbps"]
    _4mbit = 5,
}
impl From<Rateoverride> for u8 {
    #[inline(always)]
    fn from(variant: Rateoverride) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rateoverride {
    type Ux = u8;
}
impl crate::IsEnum for Rateoverride {}
#[doc = "Field `RATEOVERRIDE` reader - Data rate override setting."]
pub type RateoverrideR = crate::FieldReader<Rateoverride>;
impl RateoverrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rateoverride> {
        match self.bits {
            0 => Some(Rateoverride::_125kbit),
            2 => Some(Rateoverride::_500kbit),
            3 => Some(Rateoverride::_1mbit),
            4 => Some(Rateoverride::_2mbit),
            5 => Some(Rateoverride::_4mbit),
            _ => None,
        }
    }
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn is_125kbit(&self) -> bool {
        *self == Rateoverride::_125kbit
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn is_500kbit(&self) -> bool {
        *self == Rateoverride::_500kbit
    }
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        *self == Rateoverride::_1mbit
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        *self == Rateoverride::_2mbit
    }
    #[doc = "4 Mbps"]
    #[inline(always)]
    pub fn is_4mbit(&self) -> bool {
        *self == Rateoverride::_4mbit
    }
}
#[doc = "Field `RATEOVERRIDE` writer - Data rate override setting."]
pub type RateoverrideW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rateoverride>;
impl<'a, REG> RateoverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn _125kbit(self) -> &'a mut crate::W<REG> {
        self.variant(Rateoverride::_125kbit)
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn _500kbit(self) -> &'a mut crate::W<REG> {
        self.variant(Rateoverride::_500kbit)
    }
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Rateoverride::_1mbit)
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Rateoverride::_2mbit)
    }
    #[doc = "4 Mbps"]
    #[inline(always)]
    pub fn _4mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Rateoverride::_4mbit)
    }
}
impl R {
    #[doc = "Bits 0:2 - Data rate override setting."]
    #[inline(always)]
    pub fn rateoverride(&self) -> RateoverrideR {
        RateoverrideR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data rate override setting."]
    #[inline(always)]
    pub fn rateoverride(&mut self) -> RateoverrideW<RateoverrideSpec> {
        RateoverrideW::new(self, 0)
    }
}
#[doc = "Data rate override setting.\n\nYou can [`read`](crate::Reg::read) this register and get [`rateoverride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rateoverride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RateoverrideSpec;
impl crate::RegisterSpec for RateoverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rateoverride::R`](R) reader structure"]
impl crate::Readable for RateoverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`rateoverride::W`](W) writer structure"]
impl crate::Writable for RateoverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RATEOVERRIDE to value 0x02"]
impl crate::Resettable for RateoverrideSpec {
    const RESET_VALUE: u32 = 0x02;
}
