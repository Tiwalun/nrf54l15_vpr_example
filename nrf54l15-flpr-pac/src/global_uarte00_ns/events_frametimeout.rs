#[doc = "Register `EVENTS_FRAMETIMEOUT` reader"]
pub type R = crate::R<EventsFrametimeoutSpec>;
#[doc = "Register `EVENTS_FRAMETIMEOUT` writer"]
pub type W = crate::W<EventsFrametimeoutSpec>;
#[doc = "Timed out due to bus being idle while receiving data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFrametimeout {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFrametimeout> for bool {
    #[inline(always)]
    fn from(variant: EventsFrametimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FRAMETIMEOUT` reader - Timed out due to bus being idle while receiving data."]
pub type EventsFrametimeoutR = crate::BitReader<EventsFrametimeout>;
impl EventsFrametimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFrametimeout {
        match self.bits {
            false => EventsFrametimeout::NotGenerated,
            true => EventsFrametimeout::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFrametimeout::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFrametimeout::Generated
    }
}
#[doc = "Field `EVENTS_FRAMETIMEOUT` writer - Timed out due to bus being idle while receiving data."]
pub type EventsFrametimeoutW<'a, REG> = crate::BitWriter<'a, REG, EventsFrametimeout>;
impl<'a, REG> EventsFrametimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFrametimeout::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFrametimeout::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Timed out due to bus being idle while receiving data."]
    #[inline(always)]
    pub fn events_frametimeout(&self) -> EventsFrametimeoutR {
        EventsFrametimeoutR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timed out due to bus being idle while receiving data."]
    #[inline(always)]
    pub fn events_frametimeout(&mut self) -> EventsFrametimeoutW<EventsFrametimeoutSpec> {
        EventsFrametimeoutW::new(self, 0)
    }
}
#[doc = "Timed out due to bus being idle while receiving data.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_frametimeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_frametimeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFrametimeoutSpec;
impl crate::RegisterSpec for EventsFrametimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_frametimeout::R`](R) reader structure"]
impl crate::Readable for EventsFrametimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`events_frametimeout::W`](W) writer structure"]
impl crate::Writable for EventsFrametimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_FRAMETIMEOUT to value 0"]
impl crate::Resettable for EventsFrametimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
