#[doc = "Register `SECONDLAST` reader"]
pub type R = crate::R<SecondlastSpec>;
#[doc = "Register `SECONDLAST` writer"]
pub type W = crate::W<SecondlastSpec>;
#[doc = "Field `V` reader - NFCID1 byte V"]
pub type VR = crate::FieldReader;
#[doc = "Field `V` writer - NFCID1 byte V"]
pub type VW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `U` reader - NFCID1 byte U"]
pub type UR = crate::FieldReader;
#[doc = "Field `U` writer - NFCID1 byte U"]
pub type UW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T` reader - NFCID1 byte T"]
pub type TR = crate::FieldReader;
#[doc = "Field `T` writer - NFCID1 byte T"]
pub type TW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte V"]
    #[inline(always)]
    pub fn v(&self) -> VR {
        VR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte U"]
    #[inline(always)]
    pub fn u(&self) -> UR {
        UR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte T"]
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte V"]
    #[inline(always)]
    #[must_use]
    pub fn v(&mut self) -> VW<SecondlastSpec> {
        VW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NFCID1 byte U"]
    #[inline(always)]
    #[must_use]
    pub fn u(&mut self) -> UW<SecondlastSpec> {
        UW::new(self, 8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte T"]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> TW<SecondlastSpec> {
        TW::new(self, 16)
    }
}
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`secondlast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secondlast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecondlastSpec;
impl crate::RegisterSpec for SecondlastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secondlast::R`](R) reader structure"]
impl crate::Readable for SecondlastSpec {}
#[doc = "`write(|w| ..)` method takes [`secondlast::W`](W) writer structure"]
impl crate::Writable for SecondlastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECONDLAST to value 0"]
impl crate::Resettable for SecondlastSpec {
    const RESET_VALUE: u32 = 0;
}
