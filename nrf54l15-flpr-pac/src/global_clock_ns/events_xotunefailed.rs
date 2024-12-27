#[doc = "Register `EVENTS_XOTUNEFAILED` reader"]
pub type R = crate::R<EventsXotunefailedSpec>;
#[doc = "Register `EVENTS_XOTUNEFAILED` writer"]
pub type W = crate::W<EventsXotunefailedSpec>;
#[doc = "HFXO tuning could not be completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsXotunefailed {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsXotunefailed> for bool {
    #[inline(always)]
    fn from(variant: EventsXotunefailed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_XOTUNEFAILED` reader - HFXO tuning could not be completed"]
pub type EventsXotunefailedR = crate::BitReader<EventsXotunefailed>;
impl EventsXotunefailedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsXotunefailed {
        match self.bits {
            false => EventsXotunefailed::NotGenerated,
            true => EventsXotunefailed::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsXotunefailed::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsXotunefailed::Generated
    }
}
#[doc = "Field `EVENTS_XOTUNEFAILED` writer - HFXO tuning could not be completed"]
pub type EventsXotunefailedW<'a, REG> = crate::BitWriter<'a, REG, EventsXotunefailed>;
impl<'a, REG> EventsXotunefailedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsXotunefailed::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsXotunefailed::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - HFXO tuning could not be completed"]
    #[inline(always)]
    pub fn events_xotunefailed(&self) -> EventsXotunefailedR {
        EventsXotunefailedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXO tuning could not be completed"]
    #[inline(always)]
    #[must_use]
    pub fn events_xotunefailed(&mut self) -> EventsXotunefailedW<EventsXotunefailedSpec> {
        EventsXotunefailedW::new(self, 0)
    }
}
#[doc = "HFXO tuning could not be completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_xotunefailed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_xotunefailed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsXotunefailedSpec;
impl crate::RegisterSpec for EventsXotunefailedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_xotunefailed::R`](R) reader structure"]
impl crate::Readable for EventsXotunefailedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_xotunefailed::W`](W) writer structure"]
impl crate::Writable for EventsXotunefailedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_XOTUNEFAILED to value 0"]
impl crate::Resettable for EventsXotunefailedSpec {
    const RESET_VALUE: u32 = 0;
}
