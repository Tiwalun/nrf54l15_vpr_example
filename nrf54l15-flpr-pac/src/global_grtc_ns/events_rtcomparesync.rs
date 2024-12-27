#[doc = "Register `EVENTS_RTCOMPARESYNC` reader"]
pub type R = crate::R<EventsRtcomparesyncSpec>;
#[doc = "Register `EVENTS_RTCOMPARESYNC` writer"]
pub type W = crate::W<EventsRtcomparesyncSpec>;
#[doc = "The GRTC low frequency timer is synchronized with the SYSCOUNTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRtcomparesync {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRtcomparesync> for bool {
    #[inline(always)]
    fn from(variant: EventsRtcomparesync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RTCOMPARESYNC` reader - The GRTC low frequency timer is synchronized with the SYSCOUNTER"]
pub type EventsRtcomparesyncR = crate::BitReader<EventsRtcomparesync>;
impl EventsRtcomparesyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRtcomparesync {
        match self.bits {
            false => EventsRtcomparesync::NotGenerated,
            true => EventsRtcomparesync::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRtcomparesync::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRtcomparesync::Generated
    }
}
#[doc = "Field `EVENTS_RTCOMPARESYNC` writer - The GRTC low frequency timer is synchronized with the SYSCOUNTER"]
pub type EventsRtcomparesyncW<'a, REG> = crate::BitWriter<'a, REG, EventsRtcomparesync>;
impl<'a, REG> EventsRtcomparesyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRtcomparesync::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRtcomparesync::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The GRTC low frequency timer is synchronized with the SYSCOUNTER"]
    #[inline(always)]
    pub fn events_rtcomparesync(&self) -> EventsRtcomparesyncR {
        EventsRtcomparesyncR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The GRTC low frequency timer is synchronized with the SYSCOUNTER"]
    #[inline(always)]
    pub fn events_rtcomparesync(&mut self) -> EventsRtcomparesyncW<EventsRtcomparesyncSpec> {
        EventsRtcomparesyncW::new(self, 0)
    }
}
#[doc = "The GRTC low frequency timer is synchronized with the SYSCOUNTER\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rtcomparesync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rtcomparesync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRtcomparesyncSpec;
impl crate::RegisterSpec for EventsRtcomparesyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rtcomparesync::R`](R) reader structure"]
impl crate::Readable for EventsRtcomparesyncSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rtcomparesync::W`](W) writer structure"]
impl crate::Writable for EventsRtcomparesyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_RTCOMPARESYNC to value 0"]
impl crate::Resettable for EventsRtcomparesyncSpec {
    const RESET_VALUE: u32 = 0;
}
