#[doc = "Register `EVENTS_MEMACCERR` reader"]
pub type R = crate::R<EventsMemaccerrSpec>;
#[doc = "Register `EVENTS_MEMACCERR` writer"]
pub type W = crate::W<EventsMemaccerrSpec>;
#[doc = "Memory Access Error event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsMemaccerr {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsMemaccerr> for bool {
    #[inline(always)]
    fn from(variant: EventsMemaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_MEMACCERR` reader - Memory Access Error event"]
pub type EventsMemaccerrR = crate::BitReader<EventsMemaccerr>;
impl EventsMemaccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsMemaccerr {
        match self.bits {
            false => EventsMemaccerr::NotGenerated,
            true => EventsMemaccerr::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsMemaccerr::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsMemaccerr::Generated
    }
}
#[doc = "Field `EVENTS_MEMACCERR` writer - Memory Access Error event"]
pub type EventsMemaccerrW<'a, REG> = crate::BitWriter<'a, REG, EventsMemaccerr>;
impl<'a, REG> EventsMemaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsMemaccerr::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsMemaccerr::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Access Error event"]
    #[inline(always)]
    pub fn events_memaccerr(&self) -> EventsMemaccerrR {
        EventsMemaccerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Access Error event"]
    #[inline(always)]
    pub fn events_memaccerr(&mut self) -> EventsMemaccerrW<EventsMemaccerrSpec> {
        EventsMemaccerrW::new(self, 0)
    }
}
#[doc = "Memory Access Error event\n\nYou can [`read`](crate::Reg::read) this register and get [`events_memaccerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_memaccerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsMemaccerrSpec;
impl crate::RegisterSpec for EventsMemaccerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_memaccerr::R`](R) reader structure"]
impl crate::Readable for EventsMemaccerrSpec {}
#[doc = "`write(|w| ..)` method takes [`events_memaccerr::W`](W) writer structure"]
impl crate::Writable for EventsMemaccerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_MEMACCERR to value 0"]
impl crate::Resettable for EventsMemaccerrSpec {
    const RESET_VALUE: u32 = 0;
}
