#[doc = "Register `START` writer"]
pub type W = crate::W<StartSpec>;
#[doc = "Starts operation using easyDMA to load the values. See peripheral description for operation using easyDMA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Starts operation using easyDMA to load the values. See peripheral description for operation using easyDMA."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Starts operation using easyDMA to load the values. See peripheral description for operation using easyDMA."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<StartSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "Starts operation using easyDMA to load the values. See peripheral description for operation using easyDMA.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {
    const RESET_VALUE: u32 = 0;
}
