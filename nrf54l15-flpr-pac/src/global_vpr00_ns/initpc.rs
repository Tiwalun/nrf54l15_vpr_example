#[doc = "Register `INITPC` reader"]
pub type R = crate::R<InitpcSpec>;
#[doc = "Register `INITPC` writer"]
pub type W = crate::W<InitpcSpec>;
#[doc = "Field `INITPC` reader - Initial value of the PC at CPU start."]
pub type InitpcR = crate::FieldReader<u32>;
#[doc = "Field `INITPC` writer - Initial value of the PC at CPU start."]
pub type InitpcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initial value of the PC at CPU start."]
    #[inline(always)]
    pub fn initpc(&self) -> InitpcR {
        InitpcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initial value of the PC at CPU start."]
    #[inline(always)]
    pub fn initpc(&mut self) -> InitpcW<InitpcSpec> {
        InitpcW::new(self, 0)
    }
}
#[doc = "Initial value of the PC at CPU start.\n\nYou can [`read`](crate::Reg::read) this register and get [`initpc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initpc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitpcSpec;
impl crate::RegisterSpec for InitpcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`initpc::R`](R) reader structure"]
impl crate::Readable for InitpcSpec {}
#[doc = "`write(|w| ..)` method takes [`initpc::W`](W) writer structure"]
impl crate::Writable for InitpcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INITPC to value 0"]
impl crate::Resettable for InitpcSpec {
    const RESET_VALUE: u32 = 0;
}
