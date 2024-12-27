#[doc = "Register `FAEPEER` reader"]
pub type R = crate::R<FaepeerSpec>;
#[doc = "Register `FAEPEER` writer"]
pub type W = crate::W<FaepeerSpec>;
#[doc = "Field `FAEPEER` reader - Units 31.25 ppb."]
pub type FaepeerR = crate::FieldReader;
#[doc = "Field `FAEPEER` writer - Units 31.25 ppb."]
pub type FaepeerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Units 31.25 ppb."]
    #[inline(always)]
    pub fn faepeer(&self) -> FaepeerR {
        FaepeerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Units 31.25 ppb."]
    #[inline(always)]
    pub fn faepeer(&mut self) -> FaepeerW<FaepeerSpec> {
        FaepeerW::new(self, 0)
    }
}
#[doc = "FAEPEER (Frequency Actuation Error) of peer if known. Used during Mode 0 steps.\n\nYou can [`read`](crate::Reg::read) this register and get [`faepeer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faepeer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaepeerSpec;
impl crate::RegisterSpec for FaepeerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`faepeer::R`](R) reader structure"]
impl crate::Readable for FaepeerSpec {}
#[doc = "`write(|w| ..)` method takes [`faepeer::W`](W) writer structure"]
impl crate::Writable for FaepeerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAEPEER to value 0"]
impl crate::Resettable for FaepeerSpec {
    const RESET_VALUE: u32 = 0;
}
