#[doc = "Register `PROTECT0` reader"]
pub type R = crate::R<Protect0Spec>;
#[doc = "Register `PROTECT0` writer"]
pub type W = crate::W<Protect0Spec>;
#[doc = "\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pall {
    #[doc = "1358582010: The device can be erased using the CTRL-AP Erase all function and TAMPC PROTECT.ERASEPROTECT signal protector is unlocked."]
    Protected = 1358582010,
}
impl From<Pall> for u32 {
    #[inline(always)]
    fn from(variant: Pall) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pall {
    type Ux = u32;
}
impl crate::IsEnum for Pall {}
#[doc = "Field `PALL` reader - "]
pub type PallR = crate::FieldReader<Pall>;
impl PallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pall> {
        match self.bits {
            1358582010 => Some(Pall::Protected),
            _ => None,
        }
    }
    #[doc = "The device can be erased using the CTRL-AP Erase all function and TAMPC PROTECT.ERASEPROTECT signal protector is unlocked."]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == Pall::Protected
    }
}
#[doc = "Field `PALL` writer - "]
pub type PallW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pall>;
impl<'a, REG> PallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The device can be erased using the CTRL-AP Erase all function and TAMPC PROTECT.ERASEPROTECT signal protector is unlocked."]
    #[inline(always)]
    pub fn protected(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Protected)
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pall(&self) -> PallR {
        PallR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pall(&mut self) -> PallW<Protect0Spec> {
        PallW::new(self, 0)
    }
}
#[doc = "Description cluster: Erase protection\n\nYou can [`read`](crate::Reg::read) this register and get [`protect0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Protect0Spec;
impl crate::RegisterSpec for Protect0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`protect0::R`](R) reader structure"]
impl crate::Readable for Protect0Spec {}
#[doc = "`write(|w| ..)` method takes [`protect0::W`](W) writer structure"]
impl crate::Writable for Protect0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROTECT0 to value 0xffff_ffff"]
impl crate::Resettable for Protect0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
