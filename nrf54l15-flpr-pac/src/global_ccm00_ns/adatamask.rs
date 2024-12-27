#[doc = "Register `ADATAMASK` reader"]
pub type R = crate::R<AdatamaskSpec>;
#[doc = "Register `ADATAMASK` writer"]
pub type W = crate::W<AdatamaskSpec>;
#[doc = "Field `ADATAMASK` reader - CCM adata mask."]
pub type AdatamaskR = crate::FieldReader;
#[doc = "Field `ADATAMASK` writer - CCM adata mask."]
pub type AdatamaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CCM adata mask."]
    #[inline(always)]
    pub fn adatamask(&self) -> AdatamaskR {
        AdatamaskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CCM adata mask."]
    #[inline(always)]
    pub fn adatamask(&mut self) -> AdatamaskW<AdatamaskSpec> {
        AdatamaskW::new(self, 0)
    }
}
#[doc = "CCM adata mask.\n\nYou can [`read`](crate::Reg::read) this register and get [`adatamask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adatamask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdatamaskSpec;
impl crate::RegisterSpec for AdatamaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adatamask::R`](R) reader structure"]
impl crate::Readable for AdatamaskSpec {}
#[doc = "`write(|w| ..)` method takes [`adatamask::W`](W) writer structure"]
impl crate::Writable for AdatamaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADATAMASK to value 0xe3"]
impl crate::Resettable for AdatamaskSpec {
    const RESET_VALUE: u32 = 0xe3;
}
