#[doc = "Register `SOFTRST` reader"]
pub type R = crate::R<SoftrstSpec>;
#[doc = "Register `SOFTRST` writer"]
pub type W = crate::W<SoftrstSpec>;
#[doc = "Software reset:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softrst {
    #[doc = "0: Normal mode."]
    Normal = 0,
    #[doc = "1: The Isolated Key Generation logic and the keys are reset."]
    Key = 1,
}
impl From<Softrst> for bool {
    #[inline(always)]
    fn from(variant: Softrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTRST` reader - Software reset:"]
pub type SoftrstR = crate::BitReader<Softrst>;
impl SoftrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softrst {
        match self.bits {
            false => Softrst::Normal,
            true => Softrst::Key,
        }
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Softrst::Normal
    }
    #[doc = "The Isolated Key Generation logic and the keys are reset."]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == Softrst::Key
    }
}
#[doc = "Field `SOFTRST` writer - Software reset:"]
pub type SoftrstW<'a, REG> = crate::BitWriter<'a, REG, Softrst>;
impl<'a, REG> SoftrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Softrst::Normal)
    }
    #[doc = "The Isolated Key Generation logic and the keys are reset."]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Softrst::Key)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset:"]
    #[inline(always)]
    pub fn softrst(&self) -> SoftrstR {
        SoftrstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset:"]
    #[inline(always)]
    pub fn softrst(&mut self) -> SoftrstW<SoftrstSpec> {
        SoftrstW::new(self, 0)
    }
}
#[doc = "SoftRst register.\n\nYou can [`read`](crate::Reg::read) this register and get [`softrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstSpec;
impl crate::RegisterSpec for SoftrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst::R`](R) reader structure"]
impl crate::Readable for SoftrstSpec {}
#[doc = "`write(|w| ..)` method takes [`softrst::W`](W) writer structure"]
impl crate::Writable for SoftrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST to value 0"]
impl crate::Resettable for SoftrstSpec {
    const RESET_VALUE: u32 = 0;
}
