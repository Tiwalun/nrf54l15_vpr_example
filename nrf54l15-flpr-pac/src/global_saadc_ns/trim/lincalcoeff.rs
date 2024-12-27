#[doc = "Register `LINCALCOEFF[%s]` reader"]
pub type R = crate::R<LincalcoeffSpec>;
#[doc = "Register `LINCALCOEFF[%s]` writer"]
pub type W = crate::W<LincalcoeffSpec>;
#[doc = "Field `VAL` reader - value"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<LincalcoeffSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Description collection: Linearity calibration coefficient\n\nYou can [`read`](crate::Reg::read) this register and get [`lincalcoeff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lincalcoeff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LincalcoeffSpec;
impl crate::RegisterSpec for LincalcoeffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lincalcoeff::R`](R) reader structure"]
impl crate::Readable for LincalcoeffSpec {}
#[doc = "`write(|w| ..)` method takes [`lincalcoeff::W`](W) writer structure"]
impl crate::Writable for LincalcoeffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINCALCOEFF[%s]
to value 0"]
impl crate::Resettable for LincalcoeffSpec {
    const RESET_VALUE: u32 = 0;
}
