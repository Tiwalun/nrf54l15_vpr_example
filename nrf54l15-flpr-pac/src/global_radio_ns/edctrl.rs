#[doc = "Register `EDCTRL` reader"]
pub type R = crate::R<EdctrlSpec>;
#[doc = "Register `EDCTRL` writer"]
pub type W = crate::W<EdctrlSpec>;
#[doc = "Field `EDCNT` reader - IEEE 802.15.4 energy detect loop count"]
pub type EdcntR = crate::FieldReader<u32>;
#[doc = "Field `EDCNT` writer - IEEE 802.15.4 energy detect loop count"]
pub type EdcntW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn edcnt(&self) -> EdcntR {
        EdcntR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    #[must_use]
    pub fn edcnt(&mut self) -> EdcntW<EdctrlSpec> {
        EdcntW::new(self, 0)
    }
}
#[doc = "IEEE 802.15.4 energy detect control\n\nYou can [`read`](crate::Reg::read) this register and get [`edctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EdctrlSpec;
impl crate::RegisterSpec for EdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edctrl::R`](R) reader structure"]
impl crate::Readable for EdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`edctrl::W`](W) writer structure"]
impl crate::Writable for EdctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EDCTRL to value 0x2000_0000"]
impl crate::Resettable for EdctrlSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
