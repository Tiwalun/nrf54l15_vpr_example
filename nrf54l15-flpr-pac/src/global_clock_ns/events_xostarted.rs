#[doc = "Register `EVENTS_XOSTARTED` reader"]
pub type R = crate::R<EventsXostartedSpec>;
#[doc = "Register `EVENTS_XOSTARTED` writer"]
pub type W = crate::W<EventsXostartedSpec>;
#[doc = "Crystal oscillator has started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsXostarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsXostarted> for bool {
    #[inline(always)]
    fn from(variant: EventsXostarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_XOSTARTED` reader - Crystal oscillator has started"]
pub type EventsXostartedR = crate::BitReader<EventsXostarted>;
impl EventsXostartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsXostarted {
        match self.bits {
            false => EventsXostarted::NotGenerated,
            true => EventsXostarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsXostarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsXostarted::Generated
    }
}
#[doc = "Field `EVENTS_XOSTARTED` writer - Crystal oscillator has started"]
pub type EventsXostartedW<'a, REG> = crate::BitWriter<'a, REG, EventsXostarted>;
impl<'a, REG> EventsXostartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsXostarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsXostarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Crystal oscillator has started"]
    #[inline(always)]
    pub fn events_xostarted(&self) -> EventsXostartedR {
        EventsXostartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Crystal oscillator has started"]
    #[inline(always)]
    pub fn events_xostarted(&mut self) -> EventsXostartedW<EventsXostartedSpec> {
        EventsXostartedW::new(self, 0)
    }
}
#[doc = "Crystal oscillator has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_xostarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_xostarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsXostartedSpec;
impl crate::RegisterSpec for EventsXostartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_xostarted::R`](R) reader structure"]
impl crate::Readable for EventsXostartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_xostarted::W`](W) writer structure"]
impl crate::Writable for EventsXostartedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_XOSTARTED to value 0"]
impl crate::Resettable for EventsXostartedSpec {
    const RESET_VALUE: u32 = 0;
}
