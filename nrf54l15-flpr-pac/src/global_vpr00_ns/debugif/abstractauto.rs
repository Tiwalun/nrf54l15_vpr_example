#[doc = "Register `ABSTRACTAUTO` reader"]
pub type R = crate::R<AbstractautoSpec>;
#[doc = "Register `ABSTRACTAUTO` writer"]
pub type W = crate::W<AbstractautoSpec>;
#[doc = "Field `AUTOEXECDATA` reader - When a bit in this field is 1, read or write accesses to the corresponding data word cause the command in command to be executed again."]
pub type AutoexecdataR = crate::FieldReader<u16>;
#[doc = "Field `AUTOEXECPROGBUF` reader - When a bit in this field is 1, read or write accesses to the corresponding progbuf word cause the command in command to be executed again."]
pub type AutoexecprogbufR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - When a bit in this field is 1, read or write accesses to the corresponding data word cause the command in command to be executed again."]
    #[inline(always)]
    pub fn autoexecdata(&self) -> AutoexecdataR {
        AutoexecdataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - When a bit in this field is 1, read or write accesses to the corresponding progbuf word cause the command in command to be executed again."]
    #[inline(always)]
    pub fn autoexecprogbuf(&self) -> AutoexecprogbufR {
        AutoexecprogbufR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Abstract Command Autoexec\n\nYou can [`read`](crate::Reg::read) this register and get [`abstractauto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstractauto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AbstractautoSpec;
impl crate::RegisterSpec for AbstractautoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abstractauto::R`](R) reader structure"]
impl crate::Readable for AbstractautoSpec {}
#[doc = "`write(|w| ..)` method takes [`abstractauto::W`](W) writer structure"]
impl crate::Writable for AbstractautoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABSTRACTAUTO to value 0"]
impl crate::Resettable for AbstractautoSpec {
    const RESET_VALUE: u32 = 0;
}
