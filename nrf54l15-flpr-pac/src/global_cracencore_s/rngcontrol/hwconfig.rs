#[doc = "Register `HWCONFIG` reader"]
pub type R = crate::R<HwconfigSpec>;
#[doc = "Register `HWCONFIG` writer"]
pub type W = crate::W<HwconfigSpec>;
#[doc = "Field `NUMBOFRINGS` reader - Generic g_NumRings value."]
pub type NumbofringsR = crate::FieldReader;
#[doc = "Field `AIS31` reader - Generic g_AIS31 value."]
pub type Ais31R = crate::BitReader;
#[doc = "Field `AIS31FULL` reader - Generic g_AIS31Full value."]
pub type Ais31fullR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Generic g_NumRings value."]
    #[inline(always)]
    pub fn numbofrings(&self) -> NumbofringsR {
        NumbofringsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Generic g_AIS31 value."]
    #[inline(always)]
    pub fn ais31(&self) -> Ais31R {
        Ais31R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic g_AIS31Full value."]
    #[inline(always)]
    pub fn ais31full(&self) -> Ais31fullR {
        Ais31fullR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {}
#[doc = "Hardware configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hwconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwconfigSpec;
impl crate::RegisterSpec for HwconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwconfig::R`](R) reader structure"]
impl crate::Readable for HwconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`hwconfig::W`](W) writer structure"]
impl crate::Writable for HwconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCONFIG to value 0x0337"]
impl crate::Resettable for HwconfigSpec {
    const RESET_VALUE: u32 = 0x0337;
}
