#[doc = "Register `FREQUENCY` reader"]
pub type R = crate::R<FrequencySpec>;
#[doc = "Register `FREQUENCY` writer"]
pub type W = crate::W<FrequencySpec>;
#[doc = "Field `FREQUENCY` reader - Radio channel frequency. Frequency = 2400 + FREQUENCY (MHz)."]
pub type FrequencyR = crate::FieldReader;
#[doc = "Field `FREQUENCY` writer - Radio channel frequency. Frequency = 2400 + FREQUENCY (MHz)."]
pub type FrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MAP` reader - Channel map selection. 0: Channel map between 2400 MHZ to 2500 MHz, Frequency = 2400 + FREQUENCY (MHz). 1: Channel map between 2360 MHZ to 2460 MHz, Frequency = 2360 + FREQUENCY (MHz)."]
pub type MapR = crate::BitReader;
#[doc = "Field `MAP` writer - Channel map selection. 0: Channel map between 2400 MHZ to 2500 MHz, Frequency = 2400 + FREQUENCY (MHz). 1: Channel map between 2360 MHZ to 2460 MHz, Frequency = 2360 + FREQUENCY (MHz)."]
pub type MapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Radio channel frequency. Frequency = 2400 + FREQUENCY (MHz)."]
    #[inline(always)]
    pub fn frequency(&self) -> FrequencyR {
        FrequencyR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Channel map selection. 0: Channel map between 2400 MHZ to 2500 MHz, Frequency = 2400 + FREQUENCY (MHz). 1: Channel map between 2360 MHZ to 2460 MHz, Frequency = 2360 + FREQUENCY (MHz)."]
    #[inline(always)]
    pub fn map(&self) -> MapR {
        MapR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Radio channel frequency. Frequency = 2400 + FREQUENCY (MHz)."]
    #[inline(always)]
    pub fn frequency(&mut self) -> FrequencyW<FrequencySpec> {
        FrequencyW::new(self, 0)
    }
    #[doc = "Bit 8 - Channel map selection. 0: Channel map between 2400 MHZ to 2500 MHz, Frequency = 2400 + FREQUENCY (MHz). 1: Channel map between 2360 MHZ to 2460 MHz, Frequency = 2360 + FREQUENCY (MHz)."]
    #[inline(always)]
    pub fn map(&mut self) -> MapW<FrequencySpec> {
        MapW::new(self, 8)
    }
}
#[doc = "Frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrequencySpec;
impl crate::RegisterSpec for FrequencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frequency::R`](R) reader structure"]
impl crate::Readable for FrequencySpec {}
#[doc = "`write(|w| ..)` method takes [`frequency::W`](W) writer structure"]
impl crate::Writable for FrequencySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQUENCY to value 0x02"]
impl crate::Resettable for FrequencySpec {
    const RESET_VALUE: u32 = 0x02;
}
