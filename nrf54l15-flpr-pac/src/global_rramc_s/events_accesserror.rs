#[doc = "Register `EVENTS_ACCESSERROR` reader"]
pub type R = crate::R<EventsAccesserrorSpec>;
#[doc = "Register `EVENTS_ACCESSERROR` writer"]
pub type W = crate::W<EventsAccesserrorSpec>;
#[doc = "RRAM access error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsAccesserror {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsAccesserror> for bool {
    #[inline(always)]
    fn from(variant: EventsAccesserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ACCESSERROR` reader - RRAM access error"]
pub type EventsAccesserrorR = crate::BitReader<EventsAccesserror>;
impl EventsAccesserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsAccesserror {
        match self.bits {
            false => EventsAccesserror::NotGenerated,
            true => EventsAccesserror::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsAccesserror::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsAccesserror::Generated
    }
}
#[doc = "Field `EVENTS_ACCESSERROR` writer - RRAM access error"]
pub type EventsAccesserrorW<'a, REG> = crate::BitWriter<'a, REG, EventsAccesserror>;
impl<'a, REG> EventsAccesserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAccesserror::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAccesserror::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - RRAM access error"]
    #[inline(always)]
    pub fn events_accesserror(&self) -> EventsAccesserrorR {
        EventsAccesserrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RRAM access error"]
    #[inline(always)]
    #[must_use]
    pub fn events_accesserror(&mut self) -> EventsAccesserrorW<EventsAccesserrorSpec> {
        EventsAccesserrorW::new(self, 0)
    }
}
#[doc = "RRAM access error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_accesserror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_accesserror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsAccesserrorSpec;
impl crate::RegisterSpec for EventsAccesserrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_accesserror::R`](R) reader structure"]
impl crate::Readable for EventsAccesserrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_accesserror::W`](W) writer structure"]
impl crate::Writable for EventsAccesserrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_ACCESSERROR to value 0"]
impl crate::Resettable for EventsAccesserrorSpec {
    const RESET_VALUE: u32 = 0;
}
