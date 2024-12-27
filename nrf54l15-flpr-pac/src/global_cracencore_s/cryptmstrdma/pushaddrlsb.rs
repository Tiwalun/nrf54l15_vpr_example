#[doc = "Register `PUSHADDRLSB` reader"]
pub type R = crate::R<PushaddrlsbSpec>;
#[doc = "Register `PUSHADDRLSB` writer"]
pub type W = crate::W<PushaddrlsbSpec>;
#[doc = "Field `PUSHADDRLSB` reader - "]
pub type PushaddrlsbR = crate::FieldReader<u32>;
#[doc = "Field `PUSHADDRLSB` writer - "]
pub type PushaddrlsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pushaddrlsb(&self) -> PushaddrlsbR {
        PushaddrlsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pushaddrlsb(&mut self) -> PushaddrlsbW<PushaddrlsbSpec> {
        PushaddrlsbW::new(self, 0)
    }
}
#[doc = "Push Address Least Significant Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`pushaddrlsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pushaddrlsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PushaddrlsbSpec;
impl crate::RegisterSpec for PushaddrlsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pushaddrlsb::R`](R) reader structure"]
impl crate::Readable for PushaddrlsbSpec {}
#[doc = "`write(|w| ..)` method takes [`pushaddrlsb::W`](W) writer structure"]
impl crate::Writable for PushaddrlsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUSHADDRLSB to value 0"]
impl crate::Resettable for PushaddrlsbSpec {
    const RESET_VALUE: u32 = 0;
}
