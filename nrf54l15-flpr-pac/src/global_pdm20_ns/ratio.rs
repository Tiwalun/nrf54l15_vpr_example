#[doc = "Register `RATIO` reader"]
pub type R = crate::R<RatioSpec>;
#[doc = "Register `RATIO` writer"]
pub type W = crate::W<RatioSpec>;
#[doc = "Selects the decimation ratio between PDM_CLK and output sample rate\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ratio {
    #[doc = "0: Ratio of 32"]
    Ratio32 = 0,
    #[doc = "1: Ratio of 48"]
    Ratio48 = 1,
    #[doc = "2: Ratio of 50"]
    Ratio50 = 2,
    #[doc = "3: Ratio of 64"]
    Ratio64 = 3,
    #[doc = "4: Ratio of 80"]
    Ratio80 = 4,
    #[doc = "5: Ratio of 96"]
    Ratio96 = 5,
    #[doc = "6: Ratio of 100"]
    Ratio100 = 6,
    #[doc = "7: Ratio of 128"]
    Ratio128 = 7,
}
impl From<Ratio> for u8 {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ratio {
    type Ux = u8;
}
impl crate::IsEnum for Ratio {}
#[doc = "Field `RATIO` reader - Selects the decimation ratio between PDM_CLK and output sample rate"]
pub type RatioR = crate::FieldReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ratio {
        match self.bits {
            0 => Ratio::Ratio32,
            1 => Ratio::Ratio48,
            2 => Ratio::Ratio50,
            3 => Ratio::Ratio64,
            4 => Ratio::Ratio80,
            5 => Ratio::Ratio96,
            6 => Ratio::Ratio100,
            7 => Ratio::Ratio128,
            _ => unreachable!(),
        }
    }
    #[doc = "Ratio of 32"]
    #[inline(always)]
    pub fn is_ratio32(&self) -> bool {
        *self == Ratio::Ratio32
    }
    #[doc = "Ratio of 48"]
    #[inline(always)]
    pub fn is_ratio48(&self) -> bool {
        *self == Ratio::Ratio48
    }
    #[doc = "Ratio of 50"]
    #[inline(always)]
    pub fn is_ratio50(&self) -> bool {
        *self == Ratio::Ratio50
    }
    #[doc = "Ratio of 64"]
    #[inline(always)]
    pub fn is_ratio64(&self) -> bool {
        *self == Ratio::Ratio64
    }
    #[doc = "Ratio of 80"]
    #[inline(always)]
    pub fn is_ratio80(&self) -> bool {
        *self == Ratio::Ratio80
    }
    #[doc = "Ratio of 96"]
    #[inline(always)]
    pub fn is_ratio96(&self) -> bool {
        *self == Ratio::Ratio96
    }
    #[doc = "Ratio of 100"]
    #[inline(always)]
    pub fn is_ratio100(&self) -> bool {
        *self == Ratio::Ratio100
    }
    #[doc = "Ratio of 128"]
    #[inline(always)]
    pub fn is_ratio128(&self) -> bool {
        *self == Ratio::Ratio128
    }
}
#[doc = "Field `RATIO` writer - Selects the decimation ratio between PDM_CLK and output sample rate"]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ratio, crate::Safe>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ratio of 32"]
    #[inline(always)]
    pub fn ratio32(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio32)
    }
    #[doc = "Ratio of 48"]
    #[inline(always)]
    pub fn ratio48(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio48)
    }
    #[doc = "Ratio of 50"]
    #[inline(always)]
    pub fn ratio50(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio50)
    }
    #[doc = "Ratio of 64"]
    #[inline(always)]
    pub fn ratio64(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio64)
    }
    #[doc = "Ratio of 80"]
    #[inline(always)]
    pub fn ratio80(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio80)
    }
    #[doc = "Ratio of 96"]
    #[inline(always)]
    pub fn ratio96(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio96)
    }
    #[doc = "Ratio of 100"]
    #[inline(always)]
    pub fn ratio100(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio100)
    }
    #[doc = "Ratio of 128"]
    #[inline(always)]
    pub fn ratio128(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio128)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the decimation ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the decimation ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RatioW<RatioSpec> {
        RatioW::new(self, 0)
    }
}
#[doc = "Selects the decimation ratio between PDM_CLK and output sample rate. Change PRESCALER accordingly.\n\nYou can [`read`](crate::Reg::read) this register and get [`ratio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ratio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RatioSpec;
impl crate::RegisterSpec for RatioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ratio::R`](R) reader structure"]
impl crate::Readable for RatioSpec {}
#[doc = "`write(|w| ..)` method takes [`ratio::W`](W) writer structure"]
impl crate::Writable for RatioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RATIO to value 0x02"]
impl crate::Resettable for RatioSpec {
    const RESET_VALUE: u32 = 0x02;
}
