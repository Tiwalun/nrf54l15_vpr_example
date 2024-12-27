#[doc = "Register `EVENTS_XOTUNED` reader"]
pub type R = crate::R<EventsXotunedSpec>;
#[doc = "Register `EVENTS_XOTUNED` writer"]
pub type W = crate::W<EventsXotunedSpec>;
#[doc = "HFXO tuning is done. XOTUNED is generated after TASKS_XOSTART or after TASKS_XOTUNE has completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsXotuned {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsXotuned> for bool {
    #[inline(always)]
    fn from(variant: EventsXotuned) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_XOTUNED` reader - HFXO tuning is done. XOTUNED is generated after TASKS_XOSTART or after TASKS_XOTUNE has completed"]
pub type EventsXotunedR = crate::BitReader<EventsXotuned>;
impl EventsXotunedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsXotuned {
        match self.bits {
            false => EventsXotuned::NotGenerated,
            true => EventsXotuned::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsXotuned::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsXotuned::Generated
    }
}
#[doc = "Field `EVENTS_XOTUNED` writer - HFXO tuning is done. XOTUNED is generated after TASKS_XOSTART or after TASKS_XOTUNE has completed"]
pub type EventsXotunedW<'a, REG> = crate::BitWriter<'a, REG, EventsXotuned>;
impl<'a, REG> EventsXotunedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsXotuned::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsXotuned::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - HFXO tuning is done. XOTUNED is generated after TASKS_XOSTART or after TASKS_XOTUNE has completed"]
    #[inline(always)]
    pub fn events_xotuned(&self) -> EventsXotunedR {
        EventsXotunedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXO tuning is done. XOTUNED is generated after TASKS_XOSTART or after TASKS_XOTUNE has completed"]
    #[inline(always)]
    #[must_use]
    pub fn events_xotuned(&mut self) -> EventsXotunedW<EventsXotunedSpec> {
        EventsXotunedW::new(self, 0)
    }
}
#[doc = "HFXO tuning is done. XOTUNED is generated after TASKS_XOSTART or after TASKS_XOTUNE has completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_xotuned::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_xotuned::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsXotunedSpec;
impl crate::RegisterSpec for EventsXotunedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_xotuned::R`](R) reader structure"]
impl crate::Readable for EventsXotunedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_xotuned::W`](W) writer structure"]
impl crate::Writable for EventsXotunedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_XOTUNED to value 0"]
impl crate::Resettable for EventsXotunedSpec {
    const RESET_VALUE: u32 = 0;
}
