#[doc = "Register `EVENTS_COMPAREMATCH[%s]` reader"]
pub type R = crate::R<EventsComparematchSpec>;
#[doc = "Register `EVENTS_COMPAREMATCH[%s]` writer"]
pub type W = crate::W<EventsComparematchSpec>;
#[doc = "This event is generated when the compare matches for the compare channel \\[n\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsComparematch {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsComparematch> for bool {
    #[inline(always)]
    fn from(variant: EventsComparematch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_COMPAREMATCH` reader - This event is generated when the compare matches for the compare channel \\[n\\]."]
pub type EventsComparematchR = crate::BitReader<EventsComparematch>;
impl EventsComparematchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsComparematch {
        match self.bits {
            false => EventsComparematch::NotGenerated,
            true => EventsComparematch::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsComparematch::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsComparematch::Generated
    }
}
#[doc = "Field `EVENTS_COMPAREMATCH` writer - This event is generated when the compare matches for the compare channel \\[n\\]."]
pub type EventsComparematchW<'a, REG> = crate::BitWriter<'a, REG, EventsComparematch>;
impl<'a, REG> EventsComparematchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsComparematch::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsComparematch::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - This event is generated when the compare matches for the compare channel \\[n\\]."]
    #[inline(always)]
    pub fn events_comparematch(&self) -> EventsComparematchR {
        EventsComparematchR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This event is generated when the compare matches for the compare channel \\[n\\]."]
    #[inline(always)]
    pub fn events_comparematch(&mut self) -> EventsComparematchW<EventsComparematchSpec> {
        EventsComparematchW::new(self, 0)
    }
}
#[doc = "Description collection: This event is generated when the compare matches for the compare channel \\[n\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`events_comparematch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_comparematch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsComparematchSpec;
impl crate::RegisterSpec for EventsComparematchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_comparematch::R`](R) reader structure"]
impl crate::Readable for EventsComparematchSpec {}
#[doc = "`write(|w| ..)` method takes [`events_comparematch::W`](W) writer structure"]
impl crate::Writable for EventsComparematchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_COMPAREMATCH[%s]
to value 0"]
impl crate::Resettable for EventsComparematchSpec {
    const RESET_VALUE: u32 = 0;
}
