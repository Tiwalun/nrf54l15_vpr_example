#[doc = "Register `EVENTS_TXDONE` reader"]
pub type R = crate::R<EventsTxdoneSpec>;
#[doc = "Register `EVENTS_TXDONE` writer"]
pub type W = crate::W<EventsTxdoneSpec>;
#[doc = "TXSTATUS is changed to NoDataPending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTxdone {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTxdone> for bool {
    #[inline(always)]
    fn from(variant: EventsTxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TXDONE` reader - TXSTATUS is changed to NoDataPending."]
pub type EventsTxdoneR = crate::BitReader<EventsTxdone>;
impl EventsTxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTxdone {
        match self.bits {
            false => EventsTxdone::NotGenerated,
            true => EventsTxdone::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTxdone::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTxdone::Generated
    }
}
#[doc = "Field `EVENTS_TXDONE` writer - TXSTATUS is changed to NoDataPending."]
pub type EventsTxdoneW<'a, REG> = crate::BitWriter<'a, REG, EventsTxdone>;
impl<'a, REG> EventsTxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxdone::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxdone::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - TXSTATUS is changed to NoDataPending."]
    #[inline(always)]
    pub fn events_txdone(&self) -> EventsTxdoneR {
        EventsTxdoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXSTATUS is changed to NoDataPending."]
    #[inline(always)]
    #[must_use]
    pub fn events_txdone(&mut self) -> EventsTxdoneW<EventsTxdoneSpec> {
        EventsTxdoneW::new(self, 0)
    }
}
#[doc = "TXSTATUS is changed to NoDataPending.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxdoneSpec;
impl crate::RegisterSpec for EventsTxdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txdone::R`](R) reader structure"]
impl crate::Readable for EventsTxdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txdone::W`](W) writer structure"]
impl crate::Writable for EventsTxdoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_TXDONE to value 0"]
impl crate::Resettable for EventsTxdoneSpec {
    const RESET_VALUE: u32 = 0;
}
