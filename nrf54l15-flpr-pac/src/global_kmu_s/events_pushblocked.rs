#[doc = "Register `EVENTS_PUSHBLOCKED` reader"]
pub type R = crate::R<EventsPushblockedSpec>;
#[doc = "Register `EVENTS_PUSHBLOCKED` writer"]
pub type W = crate::W<EventsPushblockedSpec>;
#[doc = "The PUSHBLOCK operation was successful. The event is kept for backwards compatibility.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPushblocked {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPushblocked> for bool {
    #[inline(always)]
    fn from(variant: EventsPushblocked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PUSHBLOCKED` reader - The PUSHBLOCK operation was successful. The event is kept for backwards compatibility."]
pub type EventsPushblockedR = crate::BitReader<EventsPushblocked>;
impl EventsPushblockedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPushblocked {
        match self.bits {
            false => EventsPushblocked::NotGenerated,
            true => EventsPushblocked::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPushblocked::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPushblocked::Generated
    }
}
#[doc = "Field `EVENTS_PUSHBLOCKED` writer - The PUSHBLOCK operation was successful. The event is kept for backwards compatibility."]
pub type EventsPushblockedW<'a, REG> = crate::BitWriter<'a, REG, EventsPushblocked>;
impl<'a, REG> EventsPushblockedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPushblocked::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPushblocked::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The PUSHBLOCK operation was successful. The event is kept for backwards compatibility."]
    #[inline(always)]
    pub fn events_pushblocked(&self) -> EventsPushblockedR {
        EventsPushblockedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The PUSHBLOCK operation was successful. The event is kept for backwards compatibility."]
    #[inline(always)]
    pub fn events_pushblocked(&mut self) -> EventsPushblockedW<EventsPushblockedSpec> {
        EventsPushblockedW::new(self, 0)
    }
}
#[doc = "The PUSHBLOCK operation was successful. The event is kept for backwards compatibility.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pushblocked::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pushblocked::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPushblockedSpec;
impl crate::RegisterSpec for EventsPushblockedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_pushblocked::R`](R) reader structure"]
impl crate::Readable for EventsPushblockedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_pushblocked::W`](W) writer structure"]
impl crate::Writable for EventsPushblockedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_PUSHBLOCKED to value 0"]
impl crate::Resettable for EventsPushblockedSpec {
    const RESET_VALUE: u32 = 0;
}
