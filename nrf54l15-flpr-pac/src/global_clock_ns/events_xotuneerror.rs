#[doc = "Register `EVENTS_XOTUNEERROR` reader"]
pub type R = crate::R<EventsXotuneerrorSpec>;
#[doc = "Register `EVENTS_XOTUNEERROR` writer"]
pub type W = crate::W<EventsXotuneerrorSpec>;
#[doc = "HFXO quality issue detected, XOTUNE is needed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsXotuneerror {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsXotuneerror> for bool {
    #[inline(always)]
    fn from(variant: EventsXotuneerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_XOTUNEERROR` reader - HFXO quality issue detected, XOTUNE is needed"]
pub type EventsXotuneerrorR = crate::BitReader<EventsXotuneerror>;
impl EventsXotuneerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsXotuneerror {
        match self.bits {
            false => EventsXotuneerror::NotGenerated,
            true => EventsXotuneerror::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsXotuneerror::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsXotuneerror::Generated
    }
}
#[doc = "Field `EVENTS_XOTUNEERROR` writer - HFXO quality issue detected, XOTUNE is needed"]
pub type EventsXotuneerrorW<'a, REG> = crate::BitWriter<'a, REG, EventsXotuneerror>;
impl<'a, REG> EventsXotuneerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsXotuneerror::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsXotuneerror::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - HFXO quality issue detected, XOTUNE is needed"]
    #[inline(always)]
    pub fn events_xotuneerror(&self) -> EventsXotuneerrorR {
        EventsXotuneerrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXO quality issue detected, XOTUNE is needed"]
    #[inline(always)]
    pub fn events_xotuneerror(&mut self) -> EventsXotuneerrorW<EventsXotuneerrorSpec> {
        EventsXotuneerrorW::new(self, 0)
    }
}
#[doc = "HFXO quality issue detected, XOTUNE is needed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_xotuneerror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_xotuneerror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsXotuneerrorSpec;
impl crate::RegisterSpec for EventsXotuneerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_xotuneerror::R`](R) reader structure"]
impl crate::Readable for EventsXotuneerrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_xotuneerror::W`](W) writer structure"]
impl crate::Writable for EventsXotuneerrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_XOTUNEERROR to value 0"]
impl crate::Resettable for EventsXotuneerrorSpec {
    const RESET_VALUE: u32 = 0;
}
