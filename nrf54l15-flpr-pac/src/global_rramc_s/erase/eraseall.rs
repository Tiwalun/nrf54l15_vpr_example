#[doc = "Register `ERASEALL` reader"]
pub type R = crate::R<EraseallSpec>;
#[doc = "Register `ERASEALL` writer"]
pub type W = crate::W<EraseallSpec>;
#[doc = "Erase whole RRAM main block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erase {
    #[doc = "0: No operation"]
    NoOperation = 0,
    #[doc = "1: Start erase of chip"]
    Erase = 1,
}
impl From<Erase> for bool {
    #[inline(always)]
    fn from(variant: Erase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASE` reader - Erase whole RRAM main block"]
pub type EraseR = crate::BitReader<Erase>;
impl EraseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erase {
        match self.bits {
            false => Erase::NoOperation,
            true => Erase::Erase,
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == Erase::NoOperation
    }
    #[doc = "Start erase of chip"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == Erase::Erase
    }
}
#[doc = "Field `ERASE` writer - Erase whole RRAM main block"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG, Erase>;
impl<'a, REG> EraseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Erase::NoOperation)
    }
    #[doc = "Start erase of chip"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(Erase::Erase)
    }
}
impl R {
    #[doc = "Bit 0 - Erase whole RRAM main block"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase whole RRAM main block"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> EraseW<EraseallSpec> {
        EraseW::new(self, 0)
    }
}
#[doc = "Register for erasing whole RRAM main block, that includes the SICR and the UICR\n\nYou can [`read`](crate::Reg::read) this register and get [`eraseall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EraseallSpec;
impl crate::RegisterSpec for EraseallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eraseall::R`](R) reader structure"]
impl crate::Readable for EraseallSpec {}
#[doc = "`write(|w| ..)` method takes [`eraseall::W`](W) writer structure"]
impl crate::Writable for EraseallSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERASEALL to value 0"]
impl crate::Resettable for EraseallSpec {
    const RESET_VALUE: u32 = 0;
}
