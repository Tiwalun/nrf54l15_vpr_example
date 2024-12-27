#[doc = "Register `EVENTS_RAMUNDERFLOW` reader"]
pub type R = crate::R<EventsRamunderflowSpec>;
#[doc = "Register `EVENTS_RAMUNDERFLOW` writer"]
pub type W = crate::W<EventsRamunderflowSpec>;
#[doc = "Emitted when retrieving from RAM does not complete in time for the PWM module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRamunderflow {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRamunderflow> for bool {
    #[inline(always)]
    fn from(variant: EventsRamunderflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RAMUNDERFLOW` reader - Emitted when retrieving from RAM does not complete in time for the PWM module"]
pub type EventsRamunderflowR = crate::BitReader<EventsRamunderflow>;
impl EventsRamunderflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRamunderflow {
        match self.bits {
            false => EventsRamunderflow::NotGenerated,
            true => EventsRamunderflow::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRamunderflow::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRamunderflow::Generated
    }
}
#[doc = "Field `EVENTS_RAMUNDERFLOW` writer - Emitted when retrieving from RAM does not complete in time for the PWM module"]
pub type EventsRamunderflowW<'a, REG> = crate::BitWriter<'a, REG, EventsRamunderflow>;
impl<'a, REG> EventsRamunderflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRamunderflow::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRamunderflow::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Emitted when retrieving from RAM does not complete in time for the PWM module"]
    #[inline(always)]
    pub fn events_ramunderflow(&self) -> EventsRamunderflowR {
        EventsRamunderflowR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Emitted when retrieving from RAM does not complete in time for the PWM module"]
    #[inline(always)]
    #[must_use]
    pub fn events_ramunderflow(&mut self) -> EventsRamunderflowW<EventsRamunderflowSpec> {
        EventsRamunderflowW::new(self, 0)
    }
}
#[doc = "Emitted when retrieving from RAM does not complete in time for the PWM module\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ramunderflow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ramunderflow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRamunderflowSpec;
impl crate::RegisterSpec for EventsRamunderflowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ramunderflow::R`](R) reader structure"]
impl crate::Readable for EventsRamunderflowSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ramunderflow::W`](W) writer structure"]
impl crate::Writable for EventsRamunderflowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_RAMUNDERFLOW to value 0"]
impl crate::Resettable for EventsRamunderflowSpec {
    const RESET_VALUE: u32 = 0;
}
