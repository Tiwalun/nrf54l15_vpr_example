#[doc = "Register `EVENTS_WRITEERROR` reader"]
pub type R = crate::R<EventsWriteerrorSpec>;
#[doc = "Register `EVENTS_WRITEERROR` writer"]
pub type W = crate::W<EventsWriteerrorSpec>;
#[doc = "Attempt to write a VALUE in PROTECT registers without clearing the WRITEPROTECT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsWriteerror {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsWriteerror> for bool {
    #[inline(always)]
    fn from(variant: EventsWriteerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_WRITEERROR` reader - Attempt to write a VALUE in PROTECT registers without clearing the WRITEPROTECT."]
pub type EventsWriteerrorR = crate::BitReader<EventsWriteerror>;
impl EventsWriteerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsWriteerror {
        match self.bits {
            false => EventsWriteerror::NotGenerated,
            true => EventsWriteerror::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsWriteerror::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsWriteerror::Generated
    }
}
#[doc = "Field `EVENTS_WRITEERROR` writer - Attempt to write a VALUE in PROTECT registers without clearing the WRITEPROTECT."]
pub type EventsWriteerrorW<'a, REG> = crate::BitWriter<'a, REG, EventsWriteerror>;
impl<'a, REG> EventsWriteerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsWriteerror::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsWriteerror::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Attempt to write a VALUE in PROTECT registers without clearing the WRITEPROTECT."]
    #[inline(always)]
    pub fn events_writeerror(&self) -> EventsWriteerrorR {
        EventsWriteerrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Attempt to write a VALUE in PROTECT registers without clearing the WRITEPROTECT."]
    #[inline(always)]
    #[must_use]
    pub fn events_writeerror(&mut self) -> EventsWriteerrorW<EventsWriteerrorSpec> {
        EventsWriteerrorW::new(self, 0)
    }
}
#[doc = "Attempt to write a VALUE in PROTECT registers without clearing the WRITEPROTECT.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_writeerror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_writeerror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsWriteerrorSpec;
impl crate::RegisterSpec for EventsWriteerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_writeerror::R`](R) reader structure"]
impl crate::Readable for EventsWriteerrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_writeerror::W`](W) writer structure"]
impl crate::Writable for EventsWriteerrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_WRITEERROR to value 0"]
impl crate::Resettable for EventsWriteerrorSpec {
    const RESET_VALUE: u32 = 0;
}
