#[doc = "Register `PROTECT1` reader"]
pub type R = crate::R<Protect1Spec>;
#[doc = "Register `PROTECT1` writer"]
pub type W = crate::W<Protect1Spec>;
#[doc = "\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pall {
    #[doc = "4294967295: Leaves TAMPC PROTECT.DOMAIN SPIDEN and SPNIDEN signal protectors unlocked and under CPU control."]
    Unprotected = 4294967295,
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
            4294967295 => Some(Pall::Unprotected),
            _ => None,
        }
    }
    #[doc = "Leaves TAMPC PROTECT.DOMAIN SPIDEN and SPNIDEN signal protectors unlocked and under CPU control."]
    #[inline(always)]
    pub fn is_unprotected(&self) -> bool {
        *self == Pall::Unprotected
    }
}
#[doc = "Field `PALL` writer - "]
pub type PallW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pall>;
impl<'a, REG> PallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Leaves TAMPC PROTECT.DOMAIN SPIDEN and SPNIDEN signal protectors unlocked and under CPU control."]
    #[inline(always)]
    pub fn unprotected(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Unprotected)
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
    #[must_use]
    pub fn pall(&mut self) -> PallW<Protect1Spec> {
        PallW::new(self, 0)
    }
}
#[doc = "Description cluster: Access port protection register\n\nYou can [`read`](crate::Reg::read) this register and get [`protect1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Protect1Spec;
impl crate::RegisterSpec for Protect1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`protect1::R`](R) reader structure"]
impl crate::Readable for Protect1Spec {}
#[doc = "`write(|w| ..)` method takes [`protect1::W`](W) writer structure"]
impl crate::Writable for Protect1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROTECT1 to value 0xffff_ffff"]
impl crate::Resettable for Protect1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
