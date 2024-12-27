#[doc = "Register `EVENTS_PROVISIONED` reader"]
pub type R = crate::R<EventsProvisionedSpec>;
#[doc = "Register `EVENTS_PROVISIONED` writer"]
pub type W = crate::W<EventsProvisionedSpec>;
#[doc = "Key slot successfully provisioned\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsProvisioned {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsProvisioned> for bool {
    #[inline(always)]
    fn from(variant: EventsProvisioned) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PROVISIONED` reader - Key slot successfully provisioned"]
pub type EventsProvisionedR = crate::BitReader<EventsProvisioned>;
impl EventsProvisionedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsProvisioned {
        match self.bits {
            false => EventsProvisioned::NotGenerated,
            true => EventsProvisioned::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsProvisioned::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsProvisioned::Generated
    }
}
#[doc = "Field `EVENTS_PROVISIONED` writer - Key slot successfully provisioned"]
pub type EventsProvisionedW<'a, REG> = crate::BitWriter<'a, REG, EventsProvisioned>;
impl<'a, REG> EventsProvisionedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsProvisioned::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsProvisioned::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Key slot successfully provisioned"]
    #[inline(always)]
    pub fn events_provisioned(&self) -> EventsProvisionedR {
        EventsProvisionedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key slot successfully provisioned"]
    #[inline(always)]
    #[must_use]
    pub fn events_provisioned(&mut self) -> EventsProvisionedW<EventsProvisionedSpec> {
        EventsProvisionedW::new(self, 0)
    }
}
#[doc = "Key slot successfully provisioned\n\nYou can [`read`](crate::Reg::read) this register and get [`events_provisioned::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_provisioned::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsProvisionedSpec;
impl crate::RegisterSpec for EventsProvisionedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_provisioned::R`](R) reader structure"]
impl crate::Readable for EventsProvisionedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_provisioned::W`](W) writer structure"]
impl crate::Writable for EventsProvisionedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_PROVISIONED to value 0"]
impl crate::Resettable for EventsProvisionedSpec {
    const RESET_VALUE: u32 = 0;
}
