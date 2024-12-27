#[doc = "Register `REPEATTHRESHOLD` reader"]
pub type R = crate::R<RepeatthresholdSpec>;
#[doc = "Register `REPEATTHRESHOLD` writer"]
pub type W = crate::W<RepeatthresholdSpec>;
#[doc = "Field `REPEATTHRESHOLD` reader - Repetition Test Count Cut-Off value."]
pub type RepeatthresholdR = crate::FieldReader;
#[doc = "Field `REPEATTHRESHOLD` writer - Repetition Test Count Cut-Off value."]
pub type RepeatthresholdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Repetition Test Count Cut-Off value."]
    #[inline(always)]
    pub fn repeatthreshold(&self) -> RepeatthresholdR {
        RepeatthresholdR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Repetition Test Count Cut-Off value."]
    #[inline(always)]
    pub fn repeatthreshold(&mut self) -> RepeatthresholdW<RepeatthresholdSpec> {
        RepeatthresholdW::new(self, 0)
    }
}
#[doc = "Repetition Test Count Cut-Off value.\n\nYou can [`read`](crate::Reg::read) this register and get [`repeatthreshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repeatthreshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RepeatthresholdSpec;
impl crate::RegisterSpec for RepeatthresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repeatthreshold::R`](R) reader structure"]
impl crate::Readable for RepeatthresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`repeatthreshold::W`](W) writer structure"]
impl crate::Writable for RepeatthresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REPEATTHRESHOLD to value 0x29"]
impl crate::Resettable for RepeatthresholdSpec {
    const RESET_VALUE: u32 = 0x29;
}
