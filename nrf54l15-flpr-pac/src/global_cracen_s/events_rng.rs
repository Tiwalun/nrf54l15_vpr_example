#[doc = "Register `EVENTS_RNG` reader"]
pub type R = crate::R<EventsRngSpec>;
#[doc = "Register `EVENTS_RNG` writer"]
pub type W = crate::W<EventsRngSpec>;
#[doc = "Event indicating that interrupt triggered at RNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRng {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRng> for bool {
    #[inline(always)]
    fn from(variant: EventsRng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RNG` reader - Event indicating that interrupt triggered at RNG"]
pub type EventsRngR = crate::BitReader<EventsRng>;
impl EventsRngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRng {
        match self.bits {
            false => EventsRng::NotGenerated,
            true => EventsRng::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRng::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRng::Generated
    }
}
#[doc = "Field `EVENTS_RNG` writer - Event indicating that interrupt triggered at RNG"]
pub type EventsRngW<'a, REG> = crate::BitWriter<'a, REG, EventsRng>;
impl<'a, REG> EventsRngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRng::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRng::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event indicating that interrupt triggered at RNG"]
    #[inline(always)]
    pub fn events_rng(&self) -> EventsRngR {
        EventsRngR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event indicating that interrupt triggered at RNG"]
    #[inline(always)]
    #[must_use]
    pub fn events_rng(&mut self) -> EventsRngW<EventsRngSpec> {
        EventsRngW::new(self, 0)
    }
}
#[doc = "Event indicating that interrupt triggered at RNG\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRngSpec;
impl crate::RegisterSpec for EventsRngSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rng::R`](R) reader structure"]
impl crate::Readable for EventsRngSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rng::W`](W) writer structure"]
impl crate::Writable for EventsRngSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_RNG to value 0"]
impl crate::Resettable for EventsRngSpec {
    const RESET_VALUE: u32 = 0;
}
