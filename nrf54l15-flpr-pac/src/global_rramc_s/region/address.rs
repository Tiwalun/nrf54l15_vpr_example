#[doc = "Register `ADDRESS` reader"]
pub type R = crate::R<AddressSpec>;
#[doc = "Register `ADDRESS` writer"]
pub type W = crate::W<AddressSpec>;
#[doc = "Field `STARTADDR` reader - Start address of the region \\[n\\]"]
pub type StartaddrR = crate::FieldReader<u32>;
#[doc = "Field `STARTADDR` writer - Start address of the region \\[n\\]"]
pub type StartaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of the region \\[n\\]"]
    #[inline(always)]
    pub fn startaddr(&self) -> StartaddrR {
        StartaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of the region \\[n\\]"]
    #[inline(always)]
    pub fn startaddr(&mut self) -> StartaddrW<AddressSpec> {
        StartaddrW::new(self, 0)
    }
}
#[doc = "Description cluster: Region address\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddressSpec;
impl crate::RegisterSpec for AddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address::R`](R) reader structure"]
impl crate::Readable for AddressSpec {}
#[doc = "`write(|w| ..)` method takes [`address::W`](W) writer structure"]
impl crate::Writable for AddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRESS to value 0"]
impl crate::Resettable for AddressSpec {
    const RESET_VALUE: u32 = 0;
}
