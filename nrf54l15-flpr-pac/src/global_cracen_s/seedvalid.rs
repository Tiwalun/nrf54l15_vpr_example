#[doc = "Register `SEEDVALID` reader"]
pub type R = crate::R<SeedvalidSpec>;
#[doc = "Register `SEEDVALID` writer"]
pub type W = crate::W<SeedvalidSpec>;
#[doc = "Marks the SEED as valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valid {
    #[doc = "0: Valid disabled."]
    Disabled = 0,
    #[doc = "1: Valid enabled."]
    Enabled = 1,
}
impl From<Valid> for bool {
    #[inline(always)]
    fn from(variant: Valid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - Marks the SEED as valid"]
pub type ValidR = crate::BitReader<Valid>;
impl ValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Valid {
        match self.bits {
            false => Valid::Disabled,
            true => Valid::Enabled,
        }
    }
    #[doc = "Valid disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Valid::Disabled
    }
    #[doc = "Valid enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Valid::Enabled
    }
}
#[doc = "Field `VALID` writer - Marks the SEED as valid"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG, Valid>;
impl<'a, REG> ValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Valid disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::Disabled)
    }
    #[doc = "Valid enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Marks the SEED as valid"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Marks the SEED as valid"]
    #[inline(always)]
    pub fn valid(&mut self) -> ValidW<SeedvalidSpec> {
        ValidW::new(self, 0)
    }
}
#[doc = "Marks the SEED register as valid\n\nYou can [`read`](crate::Reg::read) this register and get [`seedvalid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seedvalid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeedvalidSpec;
impl crate::RegisterSpec for SeedvalidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seedvalid::R`](R) reader structure"]
impl crate::Readable for SeedvalidSpec {}
#[doc = "`write(|w| ..)` method takes [`seedvalid::W`](W) writer structure"]
impl crate::Writable for SeedvalidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEEDVALID to value 0"]
impl crate::Resettable for SeedvalidSpec {
    const RESET_VALUE: u32 = 0;
}
