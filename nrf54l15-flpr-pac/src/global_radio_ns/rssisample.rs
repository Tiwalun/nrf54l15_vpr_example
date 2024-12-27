#[doc = "Register `RSSISAMPLE` reader"]
pub type R = crate::R<RssisampleSpec>;
#[doc = "Field `RSSISAMPLE` reader - RSSI sample result. The value of this register is read as a positive value while the actual received signal strength is a negative value. Actual received signal strength is therefore as follows: received signal strength = -A dBm."]
pub type RssisampleR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - RSSI sample result. The value of this register is read as a positive value while the actual received signal strength is a negative value. Actual received signal strength is therefore as follows: received signal strength = -A dBm."]
    #[inline(always)]
    pub fn rssisample(&self) -> RssisampleR {
        RssisampleR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "RSSI sample\n\nYou can [`read`](crate::Reg::read) this register and get [`rssisample::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssisampleSpec;
impl crate::RegisterSpec for RssisampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rssisample::R`](R) reader structure"]
impl crate::Readable for RssisampleSpec {}
#[doc = "`reset()` method sets RSSISAMPLE to value 0x7f"]
impl crate::Resettable for RssisampleSpec {
    const RESET_VALUE: u32 = 0x7f;
}
