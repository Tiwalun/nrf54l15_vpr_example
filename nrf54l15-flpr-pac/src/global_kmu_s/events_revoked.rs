#[doc = "Register `EVENTS_REVOKED` reader"]
pub type R = crate::R<EventsRevokedSpec>;
#[doc = "Register `EVENTS_REVOKED` writer"]
pub type W = crate::W<EventsRevokedSpec>;
#[doc = "Key slot has been revoked and can no longer be used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRevoked {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRevoked> for bool {
    #[inline(always)]
    fn from(variant: EventsRevoked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_REVOKED` reader - Key slot has been revoked and can no longer be used"]
pub type EventsRevokedR = crate::BitReader<EventsRevoked>;
impl EventsRevokedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRevoked {
        match self.bits {
            false => EventsRevoked::NotGenerated,
            true => EventsRevoked::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRevoked::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRevoked::Generated
    }
}
#[doc = "Field `EVENTS_REVOKED` writer - Key slot has been revoked and can no longer be used"]
pub type EventsRevokedW<'a, REG> = crate::BitWriter<'a, REG, EventsRevoked>;
impl<'a, REG> EventsRevokedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRevoked::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRevoked::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Key slot has been revoked and can no longer be used"]
    #[inline(always)]
    pub fn events_revoked(&self) -> EventsRevokedR {
        EventsRevokedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key slot has been revoked and can no longer be used"]
    #[inline(always)]
    pub fn events_revoked(&mut self) -> EventsRevokedW<EventsRevokedSpec> {
        EventsRevokedW::new(self, 0)
    }
}
#[doc = "Key slot has been revoked and can no longer be used\n\nYou can [`read`](crate::Reg::read) this register and get [`events_revoked::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_revoked::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRevokedSpec;
impl crate::RegisterSpec for EventsRevokedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_revoked::R`](R) reader structure"]
impl crate::Readable for EventsRevokedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_revoked::W`](W) writer structure"]
impl crate::Writable for EventsRevokedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_REVOKED to value 0"]
impl crate::Resettable for EventsRevokedSpec {
    const RESET_VALUE: u32 = 0;
}
