#[doc = "Register `STARTADDR` reader"]
pub type R = crate::R<StartaddrSpec>;
#[doc = "Register `STARTADDR` writer"]
pub type W = crate::W<StartaddrSpec>;
#[doc = "Field `STARTADDR` reader - Start address for memory region n"]
pub type StartaddrR = crate::FieldReader<u32>;
#[doc = "Field `STARTADDR` writer - Start address for memory region n"]
pub type StartaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address for memory region n"]
    #[inline(always)]
    pub fn startaddr(&self) -> StartaddrR {
        StartaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address for memory region n"]
    #[inline(always)]
    #[must_use]
    pub fn startaddr(&mut self) -> StartaddrW<StartaddrSpec> {
        StartaddrW::new(self, 0)
    }
}
#[doc = "Description cluster: Region n start address\n\nYou can [`read`](crate::Reg::read) this register and get [`startaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartaddrSpec;
impl crate::RegisterSpec for StartaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startaddr::R`](R) reader structure"]
impl crate::Readable for StartaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`startaddr::W`](W) writer structure"]
impl crate::Writable for StartaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTADDR to value 0"]
impl crate::Resettable for StartaddrSpec {
    const RESET_VALUE: u32 = 0;
}
