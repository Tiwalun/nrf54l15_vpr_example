#[doc = "Register `SECURE` reader"]
pub type R = crate::R<SecureSpec>;
#[doc = "Register `SECURE` writer"]
pub type W = crate::W<SecureSpec>;
#[doc = "Secure port event from owner n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secure {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<Secure> for bool {
    #[inline(always)]
    fn from(variant: Secure) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECURE` reader - Secure port event from owner n"]
pub type SecureR = crate::BitReader<Secure>;
impl SecureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secure {
        match self.bits {
            false => Secure::NotGenerated,
            true => Secure::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == Secure::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == Secure::Generated
    }
}
#[doc = "Field `SECURE` writer - Secure port event from owner n"]
pub type SecureW<'a, REG> = crate::BitWriter<'a, REG, Secure>;
impl<'a, REG> SecureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Secure::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(Secure::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Secure port event from owner n"]
    #[inline(always)]
    pub fn secure(&self) -> SecureR {
        SecureR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure port event from owner n"]
    #[inline(always)]
    #[must_use]
    pub fn secure(&mut self) -> SecureW<SecureSpec> {
        SecureW::new(self, 0)
    }
}
#[doc = "Description cluster: Secure port event from owner n\n\nYou can [`read`](crate::Reg::read) this register and get [`secure::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureSpec;
impl crate::RegisterSpec for SecureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure::R`](R) reader structure"]
impl crate::Readable for SecureSpec {}
#[doc = "`write(|w| ..)` method takes [`secure::W`](W) writer structure"]
impl crate::Writable for SecureSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECURE to value 0"]
impl crate::Resettable for SecureSpec {
    const RESET_VALUE: u32 = 0;
}
