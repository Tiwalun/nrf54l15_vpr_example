#[doc = "Register `READYNEXTTIMEOUT` reader"]
pub type R = crate::R<ReadynexttimeoutSpec>;
#[doc = "Register `READYNEXTTIMEOUT` writer"]
pub type W = crate::W<ReadynexttimeoutSpec>;
#[doc = "Field `VALUE` reader - Preload value for waiting for a next write"]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - Preload value for waiting for a next write"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Enable ready next timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable ready next timeout"]
    Disable = 0,
    #[doc = "1: Enable ready next timeout"]
    Enable = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable ready next timeout"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disable,
            true => En::Enable,
        }
    }
    #[doc = "Disable ready next timeout"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == En::Disable
    }
    #[doc = "Enable ready next timeout"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == En::Enable
    }
}
#[doc = "Field `EN` writer - Enable ready next timeout"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ready next timeout"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disable)
    }
    #[doc = "Enable ready next timeout"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enable)
    }
}
impl R {
    #[doc = "Bits 0:11 - Preload value for waiting for a next write"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Enable ready next timeout"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Preload value for waiting for a next write"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<ReadynexttimeoutSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable ready next timeout"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<ReadynexttimeoutSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "Configuration for ready next timeout counter, in units of AXI clock frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`readynexttimeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readynexttimeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadynexttimeoutSpec;
impl crate::RegisterSpec for ReadynexttimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readynexttimeout::R`](R) reader structure"]
impl crate::Readable for ReadynexttimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`readynexttimeout::W`](W) writer structure"]
impl crate::Writable for ReadynexttimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets READYNEXTTIMEOUT to value 0x80"]
impl crate::Resettable for ReadynexttimeoutSpec {
    const RESET_VALUE: u32 = 0x80;
}
