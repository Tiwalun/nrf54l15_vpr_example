#[doc = "Register `LAST` reader"]
pub type R = crate::R<LastSpec>;
#[doc = "Register `LAST` writer"]
pub type W = crate::W<LastSpec>;
#[doc = "Field `Z` reader - NFCID1 byte Z (very last byte sent)"]
pub type ZR = crate::FieldReader;
#[doc = "Field `Z` writer - NFCID1 byte Z (very last byte sent)"]
pub type ZW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Y` reader - NFCID1 byte Y"]
pub type YR = crate::FieldReader;
#[doc = "Field `Y` writer - NFCID1 byte Y"]
pub type YW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `X` reader - NFCID1 byte X"]
pub type XR = crate::FieldReader;
#[doc = "Field `X` writer - NFCID1 byte X"]
pub type XW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `W` reader - NFCID1 byte W"]
pub type WR = crate::FieldReader;
#[doc = "Field `W` writer - NFCID1 byte W"]
pub type WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn z(&self) -> ZR {
        ZR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    pub fn y(&self) -> YR {
        YR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    pub fn x(&self) -> XR {
        XR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    pub fn w(&self) -> WR {
        WR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn z(&mut self) -> ZW<LastSpec> {
        ZW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    pub fn y(&mut self) -> YW<LastSpec> {
        YW::new(self, 8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    pub fn x(&mut self) -> XW<LastSpec> {
        XW::new(self, 16)
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    pub fn w(&mut self) -> WW<LastSpec> {
        WW::new(self, 24)
    }
}
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`last::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`last::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LastSpec;
impl crate::RegisterSpec for LastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`last::R`](R) reader structure"]
impl crate::Readable for LastSpec {}
#[doc = "`write(|w| ..)` method takes [`last::W`](W) writer structure"]
impl crate::Writable for LastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAST to value 0x6363"]
impl crate::Resettable for LastSpec {
    const RESET_VALUE: u32 = 0x6363;
}
