#[doc = "Register `NONSECURE` reader"]
pub type R = crate::R<NonsecureSpec>;
#[doc = "Register `NONSECURE` writer"]
pub type W = crate::W<NonsecureSpec>;
#[doc = "Non-secure port event from owner n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsecure {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<Nonsecure> for bool {
    #[inline(always)]
    fn from(variant: Nonsecure) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NONSECURE` reader - Non-secure port event from owner n"]
pub type NonsecureR = crate::BitReader<Nonsecure>;
impl NonsecureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nonsecure {
        match self.bits {
            false => Nonsecure::NotGenerated,
            true => Nonsecure::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == Nonsecure::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == Nonsecure::Generated
    }
}
#[doc = "Field `NONSECURE` writer - Non-secure port event from owner n"]
pub type NonsecureW<'a, REG> = crate::BitWriter<'a, REG, Nonsecure>;
impl<'a, REG> NonsecureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsecure::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsecure::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Non-secure port event from owner n"]
    #[inline(always)]
    pub fn nonsecure(&self) -> NonsecureR {
        NonsecureR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-secure port event from owner n"]
    #[inline(always)]
    #[must_use]
    pub fn nonsecure(&mut self) -> NonsecureW<NonsecureSpec> {
        NonsecureW::new(self, 0)
    }
}
#[doc = "Description cluster: Non-secure port event from owner n\n\nYou can [`read`](crate::Reg::read) this register and get [`nonsecure::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonsecure::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonsecureSpec;
impl crate::RegisterSpec for NonsecureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonsecure::R`](R) reader structure"]
impl crate::Readable for NonsecureSpec {}
#[doc = "`write(|w| ..)` method takes [`nonsecure::W`](W) writer structure"]
impl crate::Writable for NonsecureSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NONSECURE to value 0"]
impl crate::Resettable for NonsecureSpec {
    const RESET_VALUE: u32 = 0;
}
