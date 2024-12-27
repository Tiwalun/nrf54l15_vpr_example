#[doc = "Register `BA411EAESHWCFG2` reader"]
pub type R = crate::R<Ba411eaeshwcfg2Spec>;
#[doc = "Register `BA411EAESHWCFG2` writer"]
pub type W = crate::W<Ba411eaeshwcfg2Spec>;
#[doc = "Field `BA411EAESHWCFG2` reader - Generic g_CtrSize value."]
pub type Ba411eaeshwcfg2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Generic g_CtrSize value."]
    #[inline(always)]
    pub fn ba411eaeshwcfg2(&self) -> Ba411eaeshwcfg2R {
        Ba411eaeshwcfg2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Generic g_CtrSize value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba411eaeshwcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba411eaeshwcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ba411eaeshwcfg2Spec;
impl crate::RegisterSpec for Ba411eaeshwcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ba411eaeshwcfg2::R`](R) reader structure"]
impl crate::Readable for Ba411eaeshwcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`ba411eaeshwcfg2::W`](W) writer structure"]
impl crate::Writable for Ba411eaeshwcfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BA411EAESHWCFG2 to value 0x80"]
impl crate::Resettable for Ba411eaeshwcfg2Spec {
    const RESET_VALUE: u32 = 0x80;
}
