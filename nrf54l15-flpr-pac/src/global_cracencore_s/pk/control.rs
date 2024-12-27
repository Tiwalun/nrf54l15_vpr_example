#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `START` writer - Writing a 1 starts the processing."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARIRQ` writer - Writing a 1 clears the IRQ output."]
pub type ClearirqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 starts the processing."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<ControlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 clears the IRQ output."]
    #[inline(always)]
    #[must_use]
    pub fn clearirq(&mut self) -> ClearirqW<ControlSpec> {
        ClearirqW::new(self, 1)
    }
}
#[doc = "Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
