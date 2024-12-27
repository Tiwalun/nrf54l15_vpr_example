#[doc = "Register `PUSHLEN` reader"]
pub type R = crate::R<PushlenSpec>;
#[doc = "Register `PUSHLEN` writer"]
pub type W = crate::W<PushlenSpec>;
#[doc = "Field `PUSHLEN` reader - "]
pub type PushlenR = crate::FieldReader<u32>;
#[doc = "Field `PUSHLEN` writer - "]
pub type PushlenW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `PUSHCSTADDR` reader - "]
pub type PushcstaddrR = crate::BitReader;
#[doc = "Field `PUSHCSTADDR` writer - "]
pub type PushcstaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHREALIGN` reader - "]
pub type PushrealignR = crate::BitReader;
#[doc = "Field `PUSHREALIGN` writer - "]
pub type PushrealignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHDISCARD` reader - "]
pub type PushdiscardR = crate::BitReader;
#[doc = "Field `PUSHDISCARD` writer - "]
pub type PushdiscardW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn pushlen(&self) -> PushlenR {
        PushlenR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pushcstaddr(&self) -> PushcstaddrR {
        PushcstaddrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pushrealign(&self) -> PushrealignR {
        PushrealignR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pushdiscard(&self) -> PushdiscardR {
        PushdiscardR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    #[must_use]
    pub fn pushlen(&mut self) -> PushlenW<PushlenSpec> {
        PushlenW::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn pushcstaddr(&mut self) -> PushcstaddrW<PushlenSpec> {
        PushcstaddrW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pushrealign(&mut self) -> PushrealignW<PushlenSpec> {
        PushrealignW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pushdiscard(&mut self) -> PushdiscardW<PushlenSpec> {
        PushdiscardW::new(self, 30)
    }
}
#[doc = "Push Length\n\nYou can [`read`](crate::Reg::read) this register and get [`pushlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pushlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PushlenSpec;
impl crate::RegisterSpec for PushlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pushlen::R`](R) reader structure"]
impl crate::Readable for PushlenSpec {}
#[doc = "`write(|w| ..)` method takes [`pushlen::W`](W) writer structure"]
impl crate::Writable for PushlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUSHLEN to value 0"]
impl crate::Resettable for PushlenSpec {
    const RESET_VALUE: u32 = 0;
}
