#[doc = "Register `THIRDLAST` reader"]
pub type R = crate::R<ThirdlastSpec>;
#[doc = "Register `THIRDLAST` writer"]
pub type W = crate::W<ThirdlastSpec>;
#[doc = "Field `S` reader - NFCID1 byte S"]
pub type SR = crate::FieldReader;
#[doc = "Field `S` writer - NFCID1 byte S"]
pub type SW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R` reader - NFCID1 byte R"]
pub type RR = crate::FieldReader;
#[doc = "Field `R` writer - NFCID1 byte R"]
pub type RW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Q` reader - NFCID1 byte Q"]
pub type QR = crate::FieldReader;
#[doc = "Field `Q` writer - NFCID1 byte Q"]
pub type QW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    pub fn q(&self) -> QR {
        QR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    pub fn s(&mut self) -> SW<ThirdlastSpec> {
        SW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    pub fn r(&mut self) -> RW<ThirdlastSpec> {
        RW::new(self, 8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    pub fn q(&mut self) -> QW<ThirdlastSpec> {
        QW::new(self, 16)
    }
}
#[doc = "Third last NFCID1 part (10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`thirdlast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thirdlast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThirdlastSpec;
impl crate::RegisterSpec for ThirdlastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thirdlast::R`](R) reader structure"]
impl crate::Readable for ThirdlastSpec {}
#[doc = "`write(|w| ..)` method takes [`thirdlast::W`](W) writer structure"]
impl crate::Writable for ThirdlastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THIRDLAST to value 0"]
impl crate::Resettable for ThirdlastSpec {
    const RESET_VALUE: u32 = 0;
}
