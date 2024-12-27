#[doc = "Register `WAKETIME` reader"]
pub type R = crate::R<WaketimeSpec>;
#[doc = "Register `WAKETIME` writer"]
pub type W = crate::W<WaketimeSpec>;
#[doc = "Field `VALUE` reader - Number of LFCLK clock cycles to wake up before the next scheduled EVENTS_COMPARE event"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Number of LFCLK clock cycles to wake up before the next scheduled EVENTS_COMPARE event"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Number of LFCLK clock cycles to wake up before the next scheduled EVENTS_COMPARE event"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of LFCLK clock cycles to wake up before the next scheduled EVENTS_COMPARE event"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<WaketimeSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "GRTC wake up time.\n\nYou can [`read`](crate::Reg::read) this register and get [`waketime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`waketime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaketimeSpec;
impl crate::RegisterSpec for WaketimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waketime::R`](R) reader structure"]
impl crate::Readable for WaketimeSpec {}
#[doc = "`write(|w| ..)` method takes [`waketime::W`](W) writer structure"]
impl crate::Writable for WaketimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKETIME to value 0x01"]
impl crate::Resettable for WaketimeSpec {
    const RESET_VALUE: u32 = 0x01;
}
