#[doc = "Register `SWOFFTMRVAL` reader"]
pub type R = crate::R<SwofftmrvalSpec>;
#[doc = "Register `SWOFFTMRVAL` writer"]
pub type W = crate::W<SwofftmrvalSpec>;
#[doc = "Field `SWOFFTMRVAL` reader - Number of clk cycles to wait before stopping the rings after the FIFO is full."]
pub type SwofftmrvalR = crate::FieldReader<u16>;
#[doc = "Field `SWOFFTMRVAL` writer - Number of clk cycles to wait before stopping the rings after the FIFO is full."]
pub type SwofftmrvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of clk cycles to wait before stopping the rings after the FIFO is full."]
    #[inline(always)]
    pub fn swofftmrval(&self) -> SwofftmrvalR {
        SwofftmrvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of clk cycles to wait before stopping the rings after the FIFO is full."]
    #[inline(always)]
    pub fn swofftmrval(&mut self) -> SwofftmrvalW<SwofftmrvalSpec> {
        SwofftmrvalW::new(self, 0)
    }
}
#[doc = "Switch off timer value.\n\nYou can [`read`](crate::Reg::read) this register and get [`swofftmrval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swofftmrval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwofftmrvalSpec;
impl crate::RegisterSpec for SwofftmrvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swofftmrval::R`](R) reader structure"]
impl crate::Readable for SwofftmrvalSpec {}
#[doc = "`write(|w| ..)` method takes [`swofftmrval::W`](W) writer structure"]
impl crate::Writable for SwofftmrvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWOFFTMRVAL to value 0xffff"]
impl crate::Resettable for SwofftmrvalSpec {
    const RESET_VALUE: u32 = 0xffff;
}
