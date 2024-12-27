#[doc = "Register `REVOKE[%s]` reader"]
pub type R = crate::R<RevokeSpec>;
#[doc = "Register `REVOKE[%s]` writer"]
pub type W = crate::W<RevokeSpec>;
#[doc = "Revocation status.\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Status {
    #[doc = "4294967295: Key not revoked."]
    NotRevoked = 4294967295,
}
impl From<Status> for u32 {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Status {
    type Ux = u32;
}
impl crate::IsEnum for Status {}
#[doc = "Field `STATUS` reader - Revocation status."]
pub type StatusR = crate::FieldReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Status> {
        match self.bits {
            4294967295 => Some(Status::NotRevoked),
            _ => None,
        }
    }
    #[doc = "Key not revoked."]
    #[inline(always)]
    pub fn is_not_revoked(&self) -> bool {
        *self == Status::NotRevoked
    }
}
#[doc = "Field `STATUS` writer - Revocation status."]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, Status>;
impl<'a, REG> StatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Key not revoked."]
    #[inline(always)]
    pub fn not_revoked(self) -> &'a mut crate::W<REG> {
        self.variant(Status::NotRevoked)
    }
}
impl R {
    #[doc = "Bits 0:31 - Revocation status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Revocation status."]
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<RevokeSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "Description collection: Revocation status for RoT public key generation \\[n\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`revoke::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revoke::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevokeSpec;
impl crate::RegisterSpec for RevokeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revoke::R`](R) reader structure"]
impl crate::Readable for RevokeSpec {}
#[doc = "`write(|w| ..)` method takes [`revoke::W`](W) writer structure"]
impl crate::Writable for RevokeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REVOKE[%s]
to value 0xffff_ffff"]
impl crate::Resettable for RevokeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
