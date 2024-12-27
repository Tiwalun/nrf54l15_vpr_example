#[doc = "Register `PKECONTROL` reader"]
pub type R = crate::R<PkecontrolSpec>;
#[doc = "Register `PKECONTROL` writer"]
pub type W = crate::W<PkecontrolSpec>;
#[doc = "Field `PKESTART` writer - Start the PKE operation or trigger for Secure mode exit."]
pub type PkestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARIRQ` writer - Clear the IRQ output."]
pub type ClearirqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start the PKE operation or trigger for Secure mode exit."]
    #[inline(always)]
    #[must_use]
    pub fn pkestart(&mut self) -> PkestartW<PkecontrolSpec> {
        PkestartW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear the IRQ output."]
    #[inline(always)]
    #[must_use]
    pub fn clearirq(&mut self) -> ClearirqW<PkecontrolSpec> {
        ClearirqW::new(self, 1)
    }
}
#[doc = "PKE Control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkecontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkecontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkecontrolSpec;
impl crate::RegisterSpec for PkecontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkecontrol::R`](R) reader structure"]
impl crate::Readable for PkecontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`pkecontrol::W`](W) writer structure"]
impl crate::Writable for PkecontrolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKECONTROL to value 0"]
impl crate::Resettable for PkecontrolSpec {
    const RESET_VALUE: u32 = 0;
}
