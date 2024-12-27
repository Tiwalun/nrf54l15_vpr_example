#[doc = "Register `FIFOLEVEL` reader"]
pub type R = crate::R<FifolevelSpec>;
#[doc = "Register `FIFOLEVEL` writer"]
pub type W = crate::W<FifolevelSpec>;
#[doc = "Field `FIFOLEVEL` reader - Number of 32 bits words of random available in the FIFO."]
pub type FifolevelR = crate::FieldReader<u32>;
#[doc = "Field `FIFOLEVEL` writer - Number of 32 bits words of random available in the FIFO."]
pub type FifolevelW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of 32 bits words of random available in the FIFO."]
    #[inline(always)]
    pub fn fifolevel(&self) -> FifolevelR {
        FifolevelR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of 32 bits words of random available in the FIFO."]
    #[inline(always)]
    pub fn fifolevel(&mut self) -> FifolevelW<FifolevelSpec> {
        FifolevelW::new(self, 0)
    }
}
#[doc = "FIFO level register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifolevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifolevelSpec;
impl crate::RegisterSpec for FifolevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifolevel::R`](R) reader structure"]
impl crate::Readable for FifolevelSpec {}
#[doc = "`write(|w| ..)` method takes [`fifolevel::W`](W) writer structure"]
impl crate::Writable for FifolevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOLEVEL to value 0"]
impl crate::Resettable for FifolevelSpec {
    const RESET_VALUE: u32 = 0;
}
