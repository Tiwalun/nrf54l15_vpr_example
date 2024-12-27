#[doc = "Register `INTSTATCLR` reader"]
pub type R = crate::R<IntstatclrSpec>;
#[doc = "Register `INTSTATCLR` writer"]
pub type W = crate::W<IntstatclrSpec>;
#[doc = "Field `INTSTATCLR` writer - "]
pub type IntstatclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn intstatclr(&mut self) -> IntstatclrW<IntstatclrSpec> {
        IntstatclrW::new(self, 0)
    }
}
#[doc = "Interrupt Status Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatclrSpec;
impl crate::RegisterSpec for IntstatclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatclr::R`](R) reader structure"]
impl crate::Readable for IntstatclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intstatclr::W`](W) writer structure"]
impl crate::Writable for IntstatclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTATCLR to value 0"]
impl crate::Resettable for IntstatclrSpec {
    const RESET_VALUE: u32 = 0;
}
