#[doc = "Register `XOSC32KTRIM` reader"]
pub type R = crate::R<Xosc32ktrimSpec>;
#[doc = "Field `SLOPE` reader - Slope trim factor on twos complement form"]
pub type SlopeR = crate::FieldReader<u16>;
#[doc = "Field `OFFSET` reader - Offset trim factor on integer form"]
pub type OffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Slope trim factor on twos complement form"]
    #[inline(always)]
    pub fn slope(&self) -> SlopeR {
        SlopeR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:25 - Offset trim factor on integer form"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "XOSC32K capacitor selection trim values\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32ktrim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xosc32ktrimSpec;
impl crate::RegisterSpec for Xosc32ktrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosc32ktrim::R`](R) reader structure"]
impl crate::Readable for Xosc32ktrimSpec {}
#[doc = "`reset()` method sets XOSC32KTRIM to value 0xffff_ffff"]
impl crate::Resettable for Xosc32ktrimSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
