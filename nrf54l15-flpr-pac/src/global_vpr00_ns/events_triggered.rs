#[doc = "Register `EVENTS_TRIGGERED[%s]` reader"]
pub type R = crate::R<EventsTriggeredSpec>;
#[doc = "Register `EVENTS_TRIGGERED[%s]` writer"]
pub type W = crate::W<EventsTriggeredSpec>;
#[doc = "VPR event \\[n\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTriggered {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTriggered> for bool {
    #[inline(always)]
    fn from(variant: EventsTriggered) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TRIGGERED` reader - VPR event \\[n\\]
register"]
pub type EventsTriggeredR = crate::BitReader<EventsTriggered>;
impl EventsTriggeredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTriggered {
        match self.bits {
            false => EventsTriggered::NotGenerated,
            true => EventsTriggered::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTriggered::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTriggered::Generated
    }
}
#[doc = "Field `EVENTS_TRIGGERED` writer - VPR event \\[n\\]
register"]
pub type EventsTriggeredW<'a, REG> = crate::BitWriter<'a, REG, EventsTriggered>;
impl<'a, REG> EventsTriggeredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTriggered::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTriggered::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - VPR event \\[n\\]
register"]
    #[inline(always)]
    pub fn events_triggered(&self) -> EventsTriggeredR {
        EventsTriggeredR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VPR event \\[n\\]
register"]
    #[inline(always)]
    #[must_use]
    pub fn events_triggered(&mut self) -> EventsTriggeredW<EventsTriggeredSpec> {
        EventsTriggeredW::new(self, 0)
    }
}
#[doc = "Description collection: VPR event \\[n\\]
register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_triggered::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_triggered::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTriggeredSpec;
impl crate::RegisterSpec for EventsTriggeredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_triggered::R`](R) reader structure"]
impl crate::Readable for EventsTriggeredSpec {}
#[doc = "`write(|w| ..)` method takes [`events_triggered::W`](W) writer structure"]
impl crate::Writable for EventsTriggeredSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_TRIGGERED[%s]
to value 0"]
impl crate::Resettable for EventsTriggeredSpec {
    const RESET_VALUE: u32 = 0;
}
