#[doc = "Register `ADDRMASK` reader"]
pub type R = crate::R<AddrmaskSpec>;
#[doc = "Register `ADDRMASK` writer"]
pub type W = crate::W<AddrmaskSpec>;
#[doc = "Field `ADDRMASK` reader - Address mask for memory region n"]
pub type AddrmaskR = crate::FieldReader<u32>;
#[doc = "Field `ADDRMASK` writer - Address mask for memory region n"]
pub type AddrmaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address mask for memory region n"]
    #[inline(always)]
    pub fn addrmask(&self) -> AddrmaskR {
        AddrmaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address mask for memory region n"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> AddrmaskW<AddrmaskSpec> {
        AddrmaskW::new(self, 0)
    }
}
#[doc = "Description cluster: Select which bits of the incoming address are compared against the STARTADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`addrmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrmaskSpec;
impl crate::RegisterSpec for AddrmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrmask::R`](R) reader structure"]
impl crate::Readable for AddrmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`addrmask::W`](W) writer structure"]
impl crate::Writable for AddrmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRMASK to value 0"]
impl crate::Resettable for AddrmaskSpec {
    const RESET_VALUE: u32 = 0;
}
