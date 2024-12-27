#[doc = "Register `INITDATA` reader"]
pub type R = crate::R<InitdataSpec>;
#[doc = "Register `INITDATA` writer"]
pub type W = crate::W<InitdataSpec>;
#[doc = "Field `INITDATA` writer - Writing a 1 initialise Nonce and Personalisation_String registers counters, i.e. start writing from the 32 LSB."]
pub type InitdataW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 initialise Nonce and Personalisation_String registers counters, i.e. start writing from the 32 LSB."]
    #[inline(always)]
    pub fn initdata(&mut self) -> InitdataW<InitdataSpec> {
        InitdataW::new(self, 0)
    }
}
#[doc = "InitData register.\n\nYou can [`read`](crate::Reg::read) this register and get [`initdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitdataSpec;
impl crate::RegisterSpec for InitdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`initdata::R`](R) reader structure"]
impl crate::Readable for InitdataSpec {}
#[doc = "`write(|w| ..)` method takes [`initdata::W`](W) writer structure"]
impl crate::Writable for InitdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INITDATA to value 0"]
impl crate::Resettable for InitdataSpec {
    const RESET_VALUE: u32 = 0;
}
