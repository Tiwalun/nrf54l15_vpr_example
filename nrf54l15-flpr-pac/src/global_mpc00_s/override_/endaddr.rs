#[doc = "Register `ENDADDR` reader"]
pub type R = crate::R<EndaddrSpec>;
#[doc = "Register `ENDADDR` writer"]
pub type W = crate::W<EndaddrSpec>;
#[doc = "Field `ENDADDR` reader - End address for override region n"]
pub type EndaddrR = crate::FieldReader<u32>;
#[doc = "Field `ENDADDR` writer - End address for override region n"]
pub type EndaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - End address for override region n"]
    #[inline(always)]
    pub fn endaddr(&self) -> EndaddrR {
        EndaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - End address for override region n"]
    #[inline(always)]
    #[must_use]
    pub fn endaddr(&mut self) -> EndaddrW<EndaddrSpec> {
        EndaddrW::new(self, 0)
    }
}
#[doc = "Description cluster: Override region n End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`endaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndaddrSpec;
impl crate::RegisterSpec for EndaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endaddr::R`](R) reader structure"]
impl crate::Readable for EndaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`endaddr::W`](W) writer structure"]
impl crate::Writable for EndaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDADDR to value 0"]
impl crate::Resettable for EndaddrSpec {
    const RESET_VALUE: u32 = 0;
}
