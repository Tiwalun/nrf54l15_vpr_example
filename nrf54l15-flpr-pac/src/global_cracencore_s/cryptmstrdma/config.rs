#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `FETCHCTRLINDIRECT` reader - "]
pub type FetchctrlindirectR = crate::BitReader;
#[doc = "Field `FETCHCTRLINDIRECT` writer - "]
pub type FetchctrlindirectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHCTRLINDIRECT` reader - "]
pub type PushctrlindirectR = crate::BitReader;
#[doc = "Field `PUSHCTRLINDIRECT` writer - "]
pub type PushctrlindirectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCHSTOP` reader - "]
pub type FetchstopR = crate::BitReader;
#[doc = "Field `FETCHSTOP` writer - "]
pub type FetchstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHSTOP` reader - "]
pub type PushstopR = crate::BitReader;
#[doc = "Field `PUSHSTOP` writer - "]
pub type PushstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTRST` reader - "]
pub type SoftrstR = crate::BitReader;
#[doc = "Field `SOFTRST` writer - "]
pub type SoftrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fetchctrlindirect(&self) -> FetchctrlindirectR {
        FetchctrlindirectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pushctrlindirect(&self) -> PushctrlindirectR {
        PushctrlindirectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fetchstop(&self) -> FetchstopR {
        FetchstopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pushstop(&self) -> PushstopR {
        PushstopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn softrst(&self) -> SoftrstR {
        SoftrstR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fetchctrlindirect(&mut self) -> FetchctrlindirectW<ConfigSpec> {
        FetchctrlindirectW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pushctrlindirect(&mut self) -> PushctrlindirectW<ConfigSpec> {
        PushctrlindirectW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fetchstop(&mut self) -> FetchstopW<ConfigSpec> {
        FetchstopW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pushstop(&mut self) -> PushstopW<ConfigSpec> {
        PushstopW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn softrst(&mut self) -> SoftrstW<ConfigSpec> {
        SoftrstW::new(self, 4)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
