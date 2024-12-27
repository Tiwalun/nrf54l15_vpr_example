#[doc = "Register `EVENTS_WOKENUP` reader"]
pub type R = crate::R<EventsWokenupSpec>;
#[doc = "Register `EVENTS_WOKENUP` writer"]
pub type W = crate::W<EventsWokenupSpec>;
#[doc = "RRAMC is woken up from low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsWokenup {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsWokenup> for bool {
    #[inline(always)]
    fn from(variant: EventsWokenup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_WOKENUP` reader - RRAMC is woken up from low power mode"]
pub type EventsWokenupR = crate::BitReader<EventsWokenup>;
impl EventsWokenupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsWokenup {
        match self.bits {
            false => EventsWokenup::NotGenerated,
            true => EventsWokenup::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsWokenup::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsWokenup::Generated
    }
}
#[doc = "Field `EVENTS_WOKENUP` writer - RRAMC is woken up from low power mode"]
pub type EventsWokenupW<'a, REG> = crate::BitWriter<'a, REG, EventsWokenup>;
impl<'a, REG> EventsWokenupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsWokenup::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsWokenup::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - RRAMC is woken up from low power mode"]
    #[inline(always)]
    pub fn events_wokenup(&self) -> EventsWokenupR {
        EventsWokenupR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RRAMC is woken up from low power mode"]
    #[inline(always)]
    pub fn events_wokenup(&mut self) -> EventsWokenupW<EventsWokenupSpec> {
        EventsWokenupW::new(self, 0)
    }
}
#[doc = "RRAMC is woken up from low power mode\n\nYou can [`read`](crate::Reg::read) this register and get [`events_wokenup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_wokenup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsWokenupSpec;
impl crate::RegisterSpec for EventsWokenupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_wokenup::R`](R) reader structure"]
impl crate::Readable for EventsWokenupSpec {}
#[doc = "`write(|w| ..)` method takes [`events_wokenup::W`](W) writer structure"]
impl crate::Writable for EventsWokenupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_WOKENUP to value 0"]
impl crate::Resettable for EventsWokenupSpec {
    const RESET_VALUE: u32 = 0;
}
